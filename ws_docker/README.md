# ws_docker

```bash
docker build -f Dockerfile.dfx_base -t dfx_base_container . --progress=plain
# docker run -it --rm dfx_base_container

docker build -f Dockerfile.dfx_pj -t dfx_pj_container . --progress=plain
docker run -it --name dfx_pj --rm dfx_pj_container
docker container stop dfx_pj

# check
dfx stop && dfx start --clean --background && dfx canister create --all && dfx build && dfx canister install --all

npm run test -- --test-timeout=30000
```

## Supplements

- ./project from ../simple_e2e folder
