module {
  public type Balance = Nat;

  public type MetaData = [MetaDatum];  
  public type MetaDatum = (Text, Value);
  public type Value = { #Nat : Nat; #Int : Int; #Blob : Blob; #Text : Text };

  public type Account = {
    owner : Principal;
    subaccount : ?Subaccount;
  };
  public type Subaccount = Blob;

  public type TransferArgs = {
    from_subaccount : ?Subaccount;
    to : Account;
    amount : Balance;
    fee : ?Balance;
    memo : ?Blob;
    created_at_time : ?Nat64;
  };
  public type TransferResult = {
    #Ok : TxIndex;
    #Err : TransferError;
  };
  public type TxIndex = Nat;
  public type TimeError = {
    #TooOld;
    #CreatedInFuture : { ledger_time : Timestamp };
  };
  public type Timestamp = Nat64;
  public type TransferError = TimeError or {
    #BadFee : { expected_fee : Balance };
    #BadBurn : { min_burn_amount : Balance };
    #InsufficientFunds : { balance : Balance };
    #Duplicate : { duplicate_of : TxIndex };
    #TemporarilyUnavailable;
    #GenericError : { error_code : Nat; message : Text };
  };

  public type SupportedStandard = {
    name : Text;
    url : Text;
  };

  /// Interface for the ICRC token canister
  public type TokenInterface = actor {
    /// Returns the name of the token
    icrc1_name : shared query () -> async Text;

    /// Returns the symbol of the token
    icrc1_symbol : shared query () -> async Text;

    /// Returns the number of decimals the token uses
    icrc1_decimals : shared query () -> async Nat8;

    /// Returns the fee charged for each transfer
    icrc1_fee : shared query () -> async Balance;

    /// Returns the tokens metadata
    icrc1_metadata : shared query () -> async MetaData;

    /// Returns the total supply of the token
    icrc1_total_supply : shared query () -> async Balance;

    /// Returns the account that is allowed to mint new tokens
    icrc1_minting_account : shared query () -> async ?Account;

    /// Returns the balance of the given account
    icrc1_balance_of : shared query (Account) -> async Balance;

    /// Transfers the given amount of tokens from the sender to the recipient
    icrc1_transfer : shared (TransferArgs) -> async TransferResult;

    /// Returns the standards supported by this token's implementation
    icrc1_supported_standards : shared query () -> async [SupportedStandard];
  };
}