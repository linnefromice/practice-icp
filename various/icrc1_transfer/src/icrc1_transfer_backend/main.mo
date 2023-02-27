import T "types";

actor {
  // NOTE: why use updates_call (not query)
  // https://forum.dfinity.org/t/inter-canister-query-calls-community-consideration/6754
  public func token_info(id : Text) : async (Text, Text, Nat8) {
    let ledger = actor (id) : T.TokenInterface;
    (
      await ledger.icrc1_name(),
      await ledger.icrc1_symbol(),
      await ledger.icrc1_decimals()
    )
  };
};
