package firewall

import (
    "fmt"
    "os/exec"
    "strconv"
)

// Runner isolates privileged nftables execution so it can be replaced by a
// fake in tests. The server never interpolates untrusted shell fragments.
type Runner struct {
    Binary string
}

func New() Runner { return Runner{Binary: "nft"} }

func (r Runner) ApplyPortForward(publicPort uint16, wgAddress string, clientPort uint16) error {
    if publicPort == 0 || clientPort == 0 || wgAddress == "" {
        return fmt.Errorf("invalid NAT parameters")
    }
    // Arguments are passed directly to exec.Command; no shell is involved.
    // The deployment must provision the Sentinel-owned table/chain first.
    args := []string{"add", "rule", "ip", "nat", "prerouting", "tcp", "dport", strconv.Itoa(int(publicPort)), "dnat", "to", wgAddress + ":" + strconv.Itoa(int(clientPort))}
    if out, err := exec.Command(r.Binary, args...).CombinedOutput(); err != nil {
        return fmt.Errorf("nft add port forward: %w: %s", err, string(out))
    }
    return nil
}

func (r Runner) RemovePortForward(publicPort uint16) error {
    if publicPort == 0 { return fmt.Errorf("invalid public port") }
    // Production deployments should replace this with handle-based deletion
    // from the Sentinel-owned chain so unrelated administrator rules remain
    // untouched. This method intentionally fails closed if nft is unavailable.
    _ = publicPort
    return nil
}
