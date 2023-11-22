.PHONY: build build build_by_manual

launch:
	cargo test
	dfx stop && dfx start --background --clean
	dfx deploy backend_by_canister
	dfx deploy backend_by_cli
	dfx deploy manager
	dfx canister update-settings backend_by_canister --add-controller $(dfx canister id backend_by_canister)
	dfx canister update-settings backend_by_cli --add-controller $(dfx canister id backend_by_cli)
	dfx canister call backend_by_canister add_controller "(principal \"$(dfx canister id manager)\")"
	dfx canister call backend_by_cli add_controller "(principal \"$(dfx canister id manager)\")"

build:
	cargo test
	dfx build backend_by_canister
	dfx build backend_by_cli
	dfx build manager

# dfx canister install backend_by_cli --mode=upgrade
# dfx canister call manager upgrade_backend "(principal \"$(dfx canister id backend_by_canister)\")"
