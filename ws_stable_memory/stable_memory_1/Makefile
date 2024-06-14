.PHONY: build build build_by_manual

launch:
	dfx stop && dfx start --background --clean
	dfx deploy backend_1

check:
	dfx canister call backend_1 get_latest_player_from_heap
	dfx canister call backend_1 get_latest_player_from_stable
	dfx canister call backend_1 get_players_len_from_heap
	dfx canister call backend_1 get_players_len_from_stable

update:
	dfx canister call backend_1 add_player '("Iron MAN", 30)'
	dfx canister call backend_1 add_player '("Captain America", 90)'
	dfx canister call backend_1 add_player '("Hulk", 28)'

upgrade:
	dfx build backend_1
	dfx canister install backend_1 --mode=upgrade
