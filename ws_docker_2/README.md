# ws_docker_2

```bash
# Run Docker
docker build -t dfx_pj_2 . --progress=plain
docker run -t -p 18545:18545 -p 14943:14943 --rm dfx_pj_2

# Confirm
curl -X POST http://localhost:18545 --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":999}'
dfx ping http://localhost:14943
```
