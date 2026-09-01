package main

import (
    "crypto/rand"
    "encoding/hex"
    "encoding/json"
    "errors"
    "fmt"
    "log"
    "net/http"
    "os"
    "strconv"
    "strings"
    "sync"
    "time"
)

type Peer struct {
    ID string `json:"id"`
    Name string `json:"name"`
    PublicKey string `json:"public_key"`
    AllowedIPv4 string `json:"allowed_ipv4"`
    CreatedAt time.Time `json:"created_at"`
}

type PortLease struct {
    Port uint16 `json:"port"`
    PeerID string `json:"peer_id"`
    ExpiresAt time.Time `json:"expires_at"`
}

type Store struct { mu sync.RWMutex; peers map[string]Peer; ports map[uint16]PortLease }
func NewStore() *Store { return &Store{peers: map[string]Peer{}, ports: map[uint16]PortLease{}} }
func randomID() string { b:=make([]byte,12); if _,err:=rand.Read(b); err!=nil{return ""}; return hex.EncodeToString(b) }
func (s *Store) allocatePort(peerID string) (PortLease,error) { s.mu.Lock(); defer s.mu.Unlock(); now:=time.Now().UTC(); for n:=20000;n<=40000;n++ { p:=uint16(n); if l,ok:=s.ports[p]; ok && l.ExpiresAt.After(now){continue}; l:=PortLease{Port:p,PeerID:peerID,ExpiresAt:now.Add(30*time.Minute)}; s.ports[p]=l; return l,nil }; return PortLease{},errors.New("port pool exhausted") }
func jsonResponse(w http.ResponseWriter,status int,payload any){ w.Header().Set("Content-Type","application/json"); w.WriteHeader(status); _=json.NewEncoder(w).Encode(payload) }

func auth(token string, next http.Handler) http.Handler { return http.HandlerFunc(func(w http.ResponseWriter,r *http.Request){ if token=="" { jsonResponse(w,500,map[string]string{"error":"SENTINEL_API_TOKEN is not configured"}); return }; got:=r.Header.Get("Authorization"); if got!="Bearer "+token { jsonResponse(w,401,map[string]string{"error":"unauthorized"}); return }; next.ServeHTTP(w,r) }) }
func securityHeaders(next http.Handler) http.Handler { return http.HandlerFunc(func(w http.ResponseWriter,r *http.Request){ w.Header().Set("X-Content-Type-Options","nosniff"); w.Header().Set("Cache-Control","no-store"); next.ServeHTTP(w,r) }) }
func limitBody(next http.Handler) http.Handler { return http.HandlerFunc(func(w http.ResponseWriter,r *http.Request){ r.Body=http.MaxBytesReader(w,r.Body,64<<10); next.ServeHTTP(w,r) }) }

func main(){
    store:=NewStore(); mux:=http.NewServeMux()
    mux.HandleFunc("/healthz",func(w http.ResponseWriter,_ *http.Request){jsonResponse(w,200,map[string]string{"status":"ok","service":"sentinel-server"})})
    api:=http.NewServeMux()
    api.HandleFunc("/api/v1/peers",func(w http.ResponseWriter,r *http.Request){
        switch r.Method {
        case http.MethodGet: store.mu.RLock(); peers:=make([]Peer,0,len(store.peers)); for _,p:=range store.peers{peers=append(peers,p)}; store.mu.RUnlock(); jsonResponse(w,200,peers)
        case http.MethodPost:
            var req struct{Name,PublicKey,AllowedIPv4 string}; if err:=json.NewDecoder(r.Body).Decode(&req);err!=nil{jsonResponse(w,400,map[string]string{"error":"invalid JSON"});return}; if strings.TrimSpace(req.Name)==""||strings.TrimSpace(req.PublicKey)==""||strings.TrimSpace(req.AllowedIPv4)==""{jsonResponse(w,400,map[string]string{"error":"name, public_key and allowed_ipv4 are required"});return}; id:=randomID(); if id==""{jsonResponse(w,500,map[string]string{"error":"secure id generation failed"});return}; p:=Peer{ID:id,Name:req.Name,PublicKey:req.PublicKey,AllowedIPv4:req.AllowedIPv4,CreatedAt:time.Now().UTC()}; store.mu.Lock();store.peers[p.ID]=p;store.mu.Unlock();jsonResponse(w,201,p)
        default:w.Header().Set("Allow","GET, POST");w.WriteHeader(405)
        }
    })
    api.HandleFunc("/api/v1/p2p/port",func(w http.ResponseWriter,r *http.Request){if r.Method!=http.MethodPost{w.Header().Set("Allow","POST");w.WriteHeader(405);return}; peerID:=r.URL.Query().Get("peer_id");if peerID==""{jsonResponse(w,400,map[string]string{"error":"peer_id is required"});return};store.mu.RLock();_,ok:=store.peers[peerID];store.mu.RUnlock();if !ok{jsonResponse(w,404,map[string]string{"error":"peer not found"});return};l,err:=store.allocatePort(peerID);if err!=nil{jsonResponse(w,503,map[string]string{"error":err.Error()});return};jsonResponse(w,201,l)})
    api.HandleFunc("/api/v1/p2p/revoke",func(w http.ResponseWriter,r *http.Request){if r.Method!=http.MethodPost{w.Header().Set("Allow","POST");w.WriteHeader(405);return};n,err:=strconv.Atoi(r.URL.Query().Get("port"));if err!=nil||n<20000||n>40000{jsonResponse(w,400,map[string]string{"error":"port must be 20000-40000"});return};store.mu.Lock();delete(store.ports,uint16(n));store.mu.Unlock();jsonResponse(w,200,map[string]any{"revoked":true,"port":n})})
    mux.Handle("/",auth(os.Getenv("SENTINEL_API_TOKEN"),securityHeaders(limitBody(api))))
    addr:=os.Getenv("SENTINEL_LISTEN");if addr==""{addr=":8080"}; if os.Getenv("SENTINEL_API_TOKEN")==""{log.Fatal("SENTINEL_API_TOKEN is required")}; log.Printf("sentinel-server listening on %s",addr)
    if cert,key:=os.Getenv("SENTINEL_TLS_CERT"),os.Getenv("SENTINEL_TLS_KEY");cert!=""&&key!=""{log.Fatal(http.ListenAndServeTLS(addr,cert,key,mux))}; if os.Getenv("SENTINEL_ALLOW_PLAINTEXT")!="1"{log.Fatal(fmt.Errorf("TLS certificate/key required; set SENTINEL_ALLOW_PLAINTEXT=1 only for a trusted local development network"))}; log.Fatal(http.ListenAndServe(addr,mux))
}
