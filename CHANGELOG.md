# Changelog

## Unreleased

- Right-click a location for its menu: ping, rename, pin, hide -- or delete, for a
  location of a manually added configuration. Hidden locations leave the card and
  come back through its own "N hidden" line; how long they stay away is a setting
  (until I refresh it myself, until the next update, until I show it again), and
  pinned ones move to the top of the list with the same pin mark a pinned card
  carries. Both are keyed by the endpoint key, never by the entry id, because a
  refresh regenerates every id. The menu is one per app and as wide as its longest
  item, measured from the item text rather than left to `max-content`.
- A manually added configuration no longer pretends to be a subscription: no
  refresh (nothing fetches it), no ⋮ menu (rename, JSON and remove are subscription
  actions), only the ping. Its locations are deleted for real, and when the last
  one goes the card goes with it.
- The window stops behaving like a browser tab. Right-click no longer opens the
  WebView's own Back / Forward / Stop / Reload menu -- everywhere except editable
  fields, where Cut/Copy/Paste is what the gesture means -- and Backspace and
  Alt+arrows no longer walk page history outside a field.

- The client no longer picks a different location by itself. A refresh
  regenerates every location id, so the chosen one was re-found by its endpoint
  (`protocol:host:port:uuid`) -- and a composite JSON profile exposes its FIRST
  proxy outbound as that host, which is exactly what a provider rotates: when
  Proxen moved the primary of «США», the stored endpoint matched nothing and the
  selection fell back to the first location of the first subscription, so the
  active location jumped on its own. The selection is now re-found by endpoint
  and then by the location label inside the same subscription; a location that
  really vanished leaves nothing selected and says so instead of choosing another
  one, and its identity is kept, so an update that brings it back restores the
  choice.

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
