# Changelog

## 0.4.0

The Linux 0.4.0 interface, ported, and the grey window fixed on Windows too.

- The window no longer turns grey for good. The page runs in a WebView2 renderer
  process of its own, and when that process dies or stops answering, WebView2
  raises `ProcessFailed` and leaves an empty or frozen page. Nothing listened for
  it, so the tray and the tunnel went on running behind a dead window until the
  app was restarted. The page is now reloaded into a new renderer, at most three
  times in two minutes so a page that crashes on load cannot loop. Nothing is lost
  on the way: subscriptions and settings live in local storage and the tunnel's
  state is read back from the service.
- The shell from Linux 0.4.0: four places to swipe between -- Home, applications,
  websites, Settings -- a page that follows the finger one for one and meets a wall
  only where the strip ends, a release decided by distance or by flick, the
  neighbour mounted beside the finger instead of a strip of background, the tab
  pill floating over the pages, and a scrollbar drawn by the shell so it stays at
  the edge during a swipe.
- Split tunnelling keeps its `Apps / Websites` pill and slides the chosen half
  under it; each half scrolls and remembers its own place, and the plate follows
  the finger. Websites are written without the asterisk: a name means itself and
  its subdomains. Adding a website is a window with one field and suggestions.
  Applications keep showing the executable's name, with the full path on hover.
- Locations: a menu per location (right click or long press) with ping, rename,
  pin, hide and delete; pinned locations on top, in the order chosen in Settings;
  hiding that survives background updates and is undone by a re-import.
- The session pill under the power button reports upload and download per second
  and how long the tunnel has been up, read from the Wintun adapter's own counters.
- The interface MTU is a setting. It travels to the core's tun inbound and is
  clamped to 1280..9000 rather than reset.
- Settings are grouped by what they belong to, with tab labels, pinned-location
  order and simultaneous pings (0, the default, keeps probing every location at
  once, as before).
- One rounding family, one panel colour, a segmented control whose selection
  slides, nothing selectable except the fields the user types into.

## 0.3.2

- Show the `JSON` marker on Hysteria2 locations too. A location edited as a
  provider profile says so whatever its transport; previously only VLESS and
  friends carried it.
- Stop latency-probing UDP-only endpoints with a TCP connect. A Hysteria2,
  WireGuard, mKCP or QUIC endpoint answers no TCP handshake, so the probe could
  only ever report a timeout and every healthy Hysteria2 location looked dead.
  Those locations now say `udp only`; measuring them properly needs the proxy
  probe the Linux and Android clients have, which Windows does not expose yet.

## 0.3.1

- Keep the service runtime on stable Xray 26.3.27. The first 0.3.1 installers
  briefly bundled prerelease 26.7.28 and were replaced; transactional core
  switching to any installed official version is retained. Known trade-off:
  26.3.27 predates the Hysteria client-reuse and native-TUN UDP FullCone
  dataplane fixes, so QUIC-heavy apps on HY2 locations may remain partially
  offline.
- Restore the kill-switch setting and carry it unchanged through the GUI,
  named-pipe protocol, reconnect journal and privileged service.
- Replace the removed packet-filter backend with service-owned fail-closed
  IPv4/IPv6 routes and explicit protection against LAN DNS fallback.
- Restore Xray core management: list, download, verify, activate and remove
  official XTLS releases for Windows x64 and ARM64.
- Switch cores transactionally while connected and restore both the previous
  core and tunnel when a candidate cannot start or pass its health check.
- Keep downloaded executables in administrator-only service storage and reject
  non-official URLs, missing SHA-256 digests, oversized archives and mismatched
  executable versions.

## 0.3.0

- Verify the selected profile's effective route before switching traffic;
  optional, fallback, balancer, and chained outbounds no longer make an
  otherwise working connection fail.
- Pin the service-owned core to stable Xray 26.3.27, mark it explicitly in the
  core menu, and configure Windows TUN addresses, DNS and routes in the service.
- Filter uninstall/setup helpers from automatic app discovery and derive names
  such as Chromium from executable version metadata instead of `chrome.exe`.
- Pin native-TUN outbound sockets to the physical interface selected by the
  Windows route table, avoiding DNS/Reality loops through Hyper-V or WSL NICs.
- Discover XboxGames installations, avoid installer/cleaner executables when
  choosing the main Steam/Xbox game binary, use real game/package icons, and
  show only the executable name while retaining the full path in a tooltip.
- Import share links for every supported Xray proxy outbound: add Hysteria2,
  WireGuard, HTTP and SOCKS5 URI parsing plus standard WireGuard config files.
- Reject subscription and JSON settings that disable TLS certificate or
  hostname validation; strengthen subscription SSRF, redirect, and size checks.
- Harden the Windows service DACL and allow uninstall to continue when a
  partially removed installation no longer has a registered service.
- Remove the obsolete user-mode packet-filtering backend after
  real-system failures prevented both VPN startup and clean uninstallation.
- Keep legacy cleanup best-effort and never hold the uninstaller hostage when
  cleanup reports a warning.
- Temporarily disable the kill-switch control pending replacement enforcement;
  Xray native TUN and process/domain split routing remain.
- Discover applications from both 32-bit and 64-bit App Paths and uninstall
  registry views, extract real executable icons, and show compact readable paths.
- Keep every successful subscription refresh authoritative when quota or expiry
  metadata is omitted by the provider.
