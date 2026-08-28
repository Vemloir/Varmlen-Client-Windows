import { describe, expect, it } from "vitest";
import type { VlessServer } from "./api";
import { supportsTcpEndpointPing } from "./location-ping";

const location = (protocol: string, transport: string): VlessServer =>
  ({ protocol, transport }) as VlessServer;

describe("Windows location probe selection", () => {
  it.each([
    ["hysteria", "hysteria"],
    ["hysteria2", "hysteria2"],
    ["hy2", "hy2"],
    ["wireguard", "wireguard"],
    ["vless", "kcp"],
    ["vless", "quic"],
  ])("does not TCP-probe the UDP-only %s/%s endpoint", (protocol, transport) => {
    expect(supportsTcpEndpointPing(location(protocol, transport))).toBe(false);
  });

  it.each([
    ["vless", "tcp"],
    ["vless", "ws"],
    ["vless", "xhttp"],
    ["trojan", "tcp"],
    ["shadowsocks", "tcp"],
  ])("TCP-probes %s/%s", (protocol, transport) => {
    expect(supportsTcpEndpointPing(location(protocol, transport))).toBe(true);
  });

  it("ignores case and surrounding spaces", () => {
    expect(supportsTcpEndpointPing(location(" Hysteria ", "HYSTEROIA"))).toBe(false);
  });
});
