import type { VlessServer } from "./api";

/** Which probe a location can actually answer — the Windows mirror of the
 *  Linux client's `location-ping.ts`. The clients must agree, because a
 *  location that reads healthy in one and dead in another is a bug report
 *  either way.
 *
 *  Windows measures latency with a bare TCP connect. A Hysteria2, QUIC/KCP or
 *  WireGuard endpoint does not listen for TCP at all, so that probe can only
 *  ever "time out": it measures the absence of a protocol, not the health of
 *  the node, and every Hysteria2 location in a subscription looked unreachable.
 *  Those locations are reported as not measurable until the Windows app grows
 *  the proxy probe the Linux and Android clients use.
 */
const UDP_ONLY_PROTOCOLS = new Set(["hysteria", "hysteria2", "hy2", "wireguard"]);
const UDP_ONLY_TRANSPORTS = new Set(["hysteria", "hysteria2", "hy2", "kcp", "quic"]);

export function supportsTcpEndpointPing(
  server: Pick<VlessServer, "protocol" | "transport">,
): boolean {
  return !(
    UDP_ONLY_PROTOCOLS.has(server.protocol.trim().toLowerCase()) ||
    UDP_ONLY_TRANSPORTS.has(server.transport.trim().toLowerCase())
  );
}
