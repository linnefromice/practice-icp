# ws_docker_2

```bash
docker build -t dfx_pj_2 . --progress=plain
docker run -it --rm dfx_pj_2

dfx start --clean --background
dfx ping
dfx stop
```
