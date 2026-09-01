package wireguard

import (
    "fmt"
    "os/exec"
)

type Controller struct { Binary string }

func New() Controller { return Controller{Binary: "wg"} }

func (c Controller) Show(interfaceName string) ([]byte, error) {
    if interfaceName == "" { return nil, fmt.Errorf("interface name is required") }
    return exec.Command(c.Binary, "show", interfaceName).CombinedOutput()
}

func (c Controller) AddPeer(interfaceName, publicKey, allowedIP string) error {
    if interfaceName == "" || publicKey == "" || allowedIP == "" { return fmt.Errorf("interface, public key and allowed IP are required") }
    cmd := exec.Command(c.Binary, "set", interfaceName, "peer", publicKey, "allowed-ips", allowedIP)
    if out, err := cmd.CombinedOutput(); err != nil { return fmt.Errorf("wg set peer: %w: %s", err, string(out)) }
    return nil
}

func (c Controller) RemovePeer(interfaceName, publicKey string) error {
    if interfaceName == "" || publicKey == "" { return fmt.Errorf("interface and public key are required") }
    if out, err := exec.Command(c.Binary, "set", interfaceName, "peer", publicKey, "remove").CombinedOutput(); err != nil { return fmt.Errorf("wg remove peer: %w: %s", err, string(out)) }
    return nil
}
