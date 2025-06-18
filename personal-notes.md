# QOL
- Quakka immediately takes damage upon entering the range of a nest

## Usability issues for the editor:
- Some Indication when the level is quicksaved or loaded

# Necessary
- Different win conditions for each level
- Attacker timer only ticks when enemy in range and resets when they exit

# The last 90%
- Can only go on to the next level after completing the current
- A next level btn after completing the current

# Potential changes to bevy
- Implement the debug picker plugin to be able to be displayed in different spots
- Have the debug picker plugin appear over ui nodes

# Planning
## Different Win/Loss Conditions
Currently there is simply loss on nest destruction and on all quakkas being dead

For the levels I've made I want:
Win: All Quakkas being dead, destroying all Nests
Loss: Nest Destruction or Quakka Destruction.

There should be a
```rust
struct DeathGoal {
    card: Card,
    count_dead: u32
}
```

And in `Level`:
```rust
pub struct Level {
    pub cards: Vec<(Card, Vec2)>,
    pub starting_deckbar: Vec<Card>,
    win_condition: DeathGoal,
    lose_condition: DeathGoal
}
```

`manage_level` will read from the level's `win` and `lose` conditions. Will display them with `game_messages`.

editor will have a menu for win and lose conditions

Some indication in editor when win and lose conditions are met
