import T "types";
import Principal "mo:base/Principal";

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

  // NOT WORKING (cannot transferFrom)
  public shared({ caller }) func transfer(id : Text, to : Principal, amt : Nat) : async T.TransferResult {
    let ledger = actor (id) : T.TokenInterface;
    let msg_caller : Principal = caller;
    let args: T.TransferArgs = {
      from_subaccount = ?Principal.toBlob(msg_caller);
      to = { owner = to; subaccount = null; };
      amount = amt;
      fee = null;
      memo = null;
      created_at_time = null;
    };
    await ledger.icrc1_transfer(args)
  };
};
