package main

import (
    "crypto/rand"
    "encoding/hex"
    "encoding/json"
    "log"
    "net/http"
    "strconv"
    "sync"
    "time"
)

type Peer struct {
    ID          string    `json:"id"`
    Name        string    `json:"name"`
    PublicKey   string    `json:"public_key"`
    AllowedIPv4 string    `json:"allowed_ipv4"`
    CreatedAt   time.Time `json:"created_at"`
}

type PortLease struct {
    Port      uint16    `json:"port"`
    PeerID    string    `json:"peer_id"`
    ExpiresAt time.Time `json:"expires_at"`
}

type Store struct {
    mu    sync.RWMutex
    peers map[string]Peer
    ports map[uint16]PortLease
}

func NewStore() *Store { return &Store{peers: map[string]Peer{}, ports: map[uint16]PortLease{}} }

func randomID() string {
    b := make([]byte, 12)
    if _, err := rand.Read(b); err != nil { return "fallback" }
    return hex.EncodeToString(b)
}

func (s *Store) allocatePort(peerID string) (PortLease, error) {
    s.mu.Lock()
    defer s.mu.Unlock()
    now := time.Now()
    for port := 20000; port <= 40000; port++ {
        p := uint16(port)
        lease, used := s.ports[p]
        if used && lease.ExpiresAt.After(now) { continue }
        next := PortLease{Port: p, PeerID: peerID, ExpiresAt: now.Add(30 * time.Minute)}
        s.ports[p] = next
        return next, nil
    }
    return PortLease{}, http.ErrServerClosed
}

func jsonResponse(w http.ResponseWriter, status int, payload any) {
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(status)
    _ = json.NewEncoder(w).Encode(payload)
}

func main() {
    store := NewStore()
    mux := http.NewServeMux()

    mux.HandleFunc("/healthz", func(w http.ResponseWriter, _ *http.Request) {
        jsonResponse(w, http.StatusOK, map[string]string{"status": "ok", "service": "sentinel-server"})
    })

    mux.HandleFunc("/api/v1/peers", func(w http.ResponseWriter, r *http.Request) {
        if r.Method != http.MethodGet && r.Method != http.MethodPost {
            w.Header().Set("Allow", "GET, POST")
            w.WriteHeader(http.StatusMethodNotAllowed)
            return
        }
        if r.Method == http.MethodGet {
            store.mu.RLock(); defer store.mu.RUnlock()
            peers := make([]Peer, 0, len(store.peers))
            for _, p := range store.peers { peers = append(peers, p) }
            jsonResponse(w, http.StatusOK, peers)
            return
        }
        var req struct { Name, PublicKey, AllowedIPv4 string }
        if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
            jsonResponse(w, http.StatusBadRequest, map[string]string{"error": "invalid JSON"})
            return
        }
        if req.Name == "" || req.PublicKey == "" || req.AllowedIPv4 == "" {
            jsonResponse(w, http.StatusBadRequest, map[string]string{"error": "name, public_key and allowed_ipv4 are required"})
            return
        }
        peer := Peer{ID: randomID(), Name: req.Name, PublicKey: req.PublicKey, AllowedIPv4: req.AllowedIPv4, CreatedAt: time.Now().UTC()}
        store.mu.Lock(); store.peers[peer.ID] = peer; store.mu.Unlock()
        jsonResponse(w, http.StatusCreated, peer)
    })

    mux.HandleFunc("/api/v1/p2p/port", func(w http.ResponseWriter, r *http.Request) {
        if r.Method != http.MethodPost {
            w.Header().Set("Allow", "POST")
            w.WriteHeader(http.StatusMethodNotAllowed)
            return
        }
        peerID := r.URL.Query().Get("peer_id")
        if peerID == "" { jsonResponse(w, http.StatusBadRequest, map[string]string{"error": "peer_id is required"}); return }
        store.mu.RLock(); _, exists := store.peers[peerID]; store.mu.RUnlock()
        if !exists { jsonResponse(w, http.StatusNotFound, map[string]string{"error": "peer not found"}); return }
        lease, err := store.allocatePort(peerID)
        if err != nil { jsonResponse(w, http.StatusServiceUnavailable, map[string]string{"error": "no P2P ports available"}); return }
        jsonResponse(w, http.StatusCreated, lease)
    })

    mux.HandleFunc("/api/v1/p2p/revoke", func(w http.ResponseWriter, r *http.Request) {
        if r.Method != http.MethodPost {
            w.Header().Set("Allow", "POST")
            w.WriteHeader(http.StatusMethodNotAllowed)
            return
        }
        raw := r.URL.Query().Get("port")
        n, err := strconv.Atoi(raw)
        if err != nil || n < 1 || n > 65535 { jsonResponse(w, http.StatusBadRequest, map[string]string{"error": "valid port is required"}); return }
        store.mu.Lock(); delete(store.ports, uint16(n)); store.mu.Unlock()
        jsonResponse(w, http.StatusOK, map[string]any{"revoked": true, "port": n})
    })

    log.Println("sentinel-server listening on :8080")
    log.Fatal(http.ListenAndServe(":8080", mux))
}
