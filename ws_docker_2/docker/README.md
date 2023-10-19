# ws_docker_2

```bash
# Run Docker
docker build -f Dockerfile.base -t dfx_hardhat_node_base . --progress=plain && docker build -f Dockerfile.pj -t dfx_hardhat_node . --progress=plain
docker run -it --name dfx_hardhat_node -p 18545:18545 -p 14943:14943 --rm dfx_hardhat_node /bin/bash

## With mount
rm -rf ./.artifacts ./.outputs && cp -rp ./artifacts_csx ./.artifacts && mkdir ./.outputs
docker run -t \
  --name dfx_hardhat_node \
  -p 18545:18545 -p 14943:14943 \
  -v $PWD/.artifacts:/workspace/artifacts \
  -v $PWD/.outputs:/workspace/outputs \
  --rm dfx_hardhat_node

### docker run -it --name dfx_hardhat_node -p 18545:18545 -p 14943:14943 -v $PWD/.artifacts:/workspace/artifacts -v $PWD/.outputs:/workspace/outputs --rm dfx_hardhat_node /bin/bash

# Confirm
curl -X POST http://localhost:18545 --data '{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":31337}'
curl -X POST http://localhost:18545 --data '{"jsonrpc":"2.0","method":"net_version","params":[],"id":31337}'
dfx ping http://localhost:14943

# Memo
dfx identity new operator --storage-mode plaintext
dfx identity use operator
```
