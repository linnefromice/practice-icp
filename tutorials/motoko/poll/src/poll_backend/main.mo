import RBTree "mo:base/RBTree";
import Nat "mo:base/Nat";
import Iter "mo:base/Iter";

actor {
  var question: Text = "enter your question";

  public query func getQuestion(): async Text {
    question
  };

  public func setQuestion(q : Text) {
    question := q
  };

  var options: RBTree.RBTree<Nat, Text> = RBTree.RBTree(Nat.compare);
  var optionCounter = 0;

  // OPTIONS
  public func addOption(newOption : Text): async Nat {
    optionCounter += 1;
    options.put(optionCounter, newOption);
    optionCounter
  };

  public func deleteOption(optionId : Nat) {
    options.delete(optionId);
  };

  public func updateOption(optionId : Nat, option : Text) {
    options.put(optionId, option);
  };

  public query func getOptions() : async [(Nat, Text)] {
    Iter.toArray(options.entries())
  };
}