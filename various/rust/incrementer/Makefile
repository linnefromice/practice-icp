.PHONY: local

local:
	dfx canister create backend --network http://localhost:$(port)
	dfx build backend --network http://localhost:$(port)
	dfx canister install backend --network http://localhost:$(port)
