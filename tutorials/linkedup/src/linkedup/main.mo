import Connected "canister:connected";
import Database "database";
import Types "types";
import Utils "utils";

actor LinkedUp {
  var directory: Database.Directory = Database.Directory();

  type NewProfile = Types.NewProfile;
  type Profile = Types.Profile;
  type UserId = Types.UserId;

  // Healthcheck
  public func healthcheck(): async Bool { true };

  // Profiles
  public shared(msg) func create(profile: NewProfile): async () {
    directory.createOne(msg.caller, profile);
  };

  public shared(msg) func update(profile: Profile): async () {
    if (Utils.hasAccess(msg.caller, profile)) {
      directory.updateOne(msg.caller, profile);
    };
  };

  public query func get(userId: UserId): async Profile {
    Utils.getProfile(directory, userId)
  };

  public query func search(term: Text): async [Profile] {
    directory.findBy(term)
  };

  // Connections
  public shared(msg) func connect(userId: UserId): async () {
    // Call Connectd's public methods without an API
    await Connected.connect(msg.caller, userId);
  };

  public func getConnections(userId: UserId): async [Profile] {
    let userIds = await Connected.getConnections(userId);
    directory.findMany(userIds)
  };

  public shared(msg) func isConnected(userId: UserId): async Bool {
    let userIds = await Connected.getConnections(msg.caller);
    Utils.includes(userId, userIds)
  };

  // User Auth
  public shared query(msg) func getOwnId(): async UserId { msg.caller }
}