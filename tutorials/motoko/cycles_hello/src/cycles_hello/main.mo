import Cycles "mo:base/ExperimentalCycles";
import Nat64 "mo:base/Nat64";
import Principal "mo:base/Principal";

shared(msg) actor class HelloCycles (
  capacity: Nat,
) {
  var balance = 0;

  public shared(msg) func wallet_balance() : async Nat {
    return balance;
  };

  // Return the cycles received up to the capacity allowed
  public func wallet_receive() : async { accepted: Nat64 } {
    let amount = Cycles.available();
    let limit : Nat = capacity - balance;
    let accepted = if (amount < limit) amount else limit;
    let deposit = Cycles.accept(accepted);
    assert (deposit == accepted);
    balance += accepted;
    { accepted = Nat64.fromNat(accepted) };
  };

  // Return the principal of the caller/user identity
  public shared(msg) func owner() : async Principal {
    let currentOwner = msg.caller;
    return currentOwner;
  }
}