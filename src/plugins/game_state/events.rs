/// Event fired when the game state begins the [PreGamePhase]
pub struct PreGameStartEvent;

/// Event fired when the game state finishes the [PreGamePhase]
pub struct PreGameEndEvent;

/// Event fired when the game state starts the [RunningGamePhase]
pub struct RunningGameStartEvent;

/// Event fired when the game state ends the [RunningGamePhase]
///
/// TODO: Maybe figure out a better way to fire this rather than directly from the game logic
pub struct RunningGameEndEvent;

/// Event fired when the game state begins the [PostGamePhase]
pub struct PostGameStartEvent;

/// Event fired when the game state finishes the [PostGamePhase]
pub struct PostGameEndEvent;