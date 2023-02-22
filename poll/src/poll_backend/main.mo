actor {
  var question: Text = "enter your question";

  public query func getQuestion(): async Text {
    question
  };

  public func setQuestion(q: Text) {
    question := q
  };
}