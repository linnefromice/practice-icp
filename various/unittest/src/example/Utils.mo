import T "./Types";

module {
  public func sum (args: T.SumArgs) : Nat {
    let (a, b) = args;
    return a + b;
  };
}
