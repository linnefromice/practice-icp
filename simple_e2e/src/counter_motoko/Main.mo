actor Counter {

  stable var counter = 0;

  // Get canister label name.
  public query func name() : async Text {
    return "CounterMotoko";
  };

  // Get the value of the counter.
  public query func get() : async Nat {
    return counter;
  };

  // Set the value of the counter.
  public func set(n : Nat) : async () {
    counter := n;
  };

  // Increment the value of the counter.
  public func inc() : async () {
    counter += 1;
  };

  // Reset the value of the counter.
  public func reset() : async () {
    counter := 0;
  };
};
