//! Bring the window back when WebView2 loses the page under it.
//!
//! The UI runs in a WebView2 renderer process of its own. When that process dies or
//! stops answering, WebView2 raises `ProcessFailed` and leaves an empty or frozen
//! page behind. Nothing was listening, so the app, the tray and the tunnel kept
//! running behind a window that had turned grey for good and only came back on a
//! restart. The Linux client had the same hole with WebKitGTK's web process.
//!
//! A reload is what asks WebView2 for a new renderer. The page owns no state that a
//! reload loses: subscriptions and settings live in localStorage, and the layout asks
//! the service for the tunnel's real state the moment it mounts.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// How many reloads a burst of failures may cost before we stop asking for more.
const MAX_RELOADS: usize = 3;
/// The window those reloads are counted over. A page that dies again as soon as it
/// is reloaded would otherwise spin a new renderer forever.
const WINDOW: Duration = Duration::from_secs(120);

/// Remembers recent reloads, so a page that crashes on load does not loop.
#[derive(Debug, Default)]
#[cfg_attr(not(windows), allow(dead_code))]
pub(crate) struct ReloadBudget {
    recent: VecDeque<Instant>,
}

#[cfg_attr(not(windows), allow(dead_code))]
impl ReloadBudget {
    /// Whether a reload may happen at `now`; records it when it may.
    pub(crate) fn take(&mut self, now: Instant) -> bool {
        while let Some(&first) = self.recent.front() {
            if now.duration_since(first) >= WINDOW {
                self.recent.pop_front();
            } else {
                break;
            }
        }
        if self.recent.len() >= MAX_RELOADS {
            return false;
        }
        self.recent.push_back(now);
        true
    }
}

/// Watch the main window's renderer and reload the page when it fails.
#[cfg(windows)]
pub fn install(app: &tauri::AppHandle) {
    use tauri::Manager;
    use webview2_com::Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_PROCESS_FAILED_KIND, COREWEBVIEW2_PROCESS_FAILED_KIND_BROWSER_PROCESS_EXITED,
        COREWEBVIEW2_PROCESS_FAILED_KIND_RENDER_PROCESS_EXITED,
        COREWEBVIEW2_PROCESS_FAILED_KIND_RENDER_PROCESS_UNRESPONSIVE,
    };
    use webview2_com::ProcessFailedEventHandler;

    let Some(window) = app.get_webview_window("main") else {
        return;
    };
    let result = window.with_webview(|webview| unsafe {
        let core = match webview.controller().CoreWebView2() {
            Ok(core) => core,
            Err(e) => {
                eprintln!("webview: no WebView2 to watch: {e}");
                return;
            }
        };
        let mut budget = ReloadBudget::default();
        let handler = ProcessFailedEventHandler::create(Box::new(move |sender, args| {
            let (Some(sender), Some(args)) = (sender, args) else {
                return Ok(());
            };
            let mut kind = COREWEBVIEW2_PROCESS_FAILED_KIND::default();
            args.ProcessFailedKind(&mut kind)?;
            // Only the page's own renderer can be brought back with a reload. A
            // subframe renderer is not ours (there are no frames), and when the
            // browser process itself is gone there is nothing left to reload into.
            if kind == COREWEBVIEW2_PROCESS_FAILED_KIND_BROWSER_PROCESS_EXITED {
                eprintln!("webview: WebView2 browser process exited");
                return Ok(());
            }
            if kind != COREWEBVIEW2_PROCESS_FAILED_KIND_RENDER_PROCESS_EXITED
                && kind != COREWEBVIEW2_PROCESS_FAILED_KIND_RENDER_PROCESS_UNRESPONSIVE
            {
                return Ok(());
            }
            if budget.take(Instant::now()) {
                eprintln!("webview: renderer failed (kind {}); reloading", kind.0);
                sender.Reload()?;
            } else {
                eprintln!(
                    "webview: renderer failed (kind {}) {MAX_RELOADS} times in {}s; not reloading again",
                    kind.0,
                    WINDOW.as_secs()
                );
            }
            Ok(())
        }));
        let mut token = 0i64;
        if let Err(e) = core.add_ProcessFailed(&handler, &mut token) {
            eprintln!("webview: could not watch the renderer: {e}");
        }
    });
    if let Err(e) = result {
        eprintln!("webview: could not watch the renderer: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_failure_now_and_then_is_always_reloaded() {
        let mut budget = ReloadBudget::default();
        let start = Instant::now();
        for i in 0..10 {
            assert!(budget.take(start + WINDOW * i), "reload {i}");
        }
    }

    #[test]
    fn a_failure_loop_stops_after_the_budget() {
        let mut budget = ReloadBudget::default();
        let start = Instant::now();
        for i in 0..MAX_RELOADS as u32 {
            assert!(budget.take(start + Duration::from_secs(i as u64)));
        }
        assert!(!budget.take(start + Duration::from_secs(10)));
        // A refused reload is not counted, so the budget still refills on time.
        assert!(budget.take(start + WINDOW + Duration::from_secs(1)));
    }
}
