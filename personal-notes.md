- Editing the win conditions in editor doesn't update WinLossProgress for somme reason
- SOUUUUUUUUNDDDD
- Have a diagram showing the win/loss condition instead of just text
- Feedback on whether a level is "reset"
- Still able to restart if paused in level
- A warning if a level is impossible to lose or win
- Have entities avoid other entities when pathfinding (boid like behavior)
- Have a visual of what kinds of enemies to slay / left alive. Perhaps the entities that need to be kept alive are in green and those that need killing are red.
- Nests won't suddenly change target if another quakka comes into contact
- Waterballs placed during pausing should work, and not just sit there. Is also not cleaned up properly.
- Can only go on to the next level after completing the current
- A next level btn after completing the current
- Rewrite the pathfinding to just do your own thing with rays, world is simple enough to do it yourself

# Potential changes to bevy
- Implement the debug picker plugin to be able to be displayed in different spots
- Have the debug picker plugin appear over ui nodes
- DragStart and Drag pointer events are a bit icky. DragStart reports in percentage but Drag is in pixels.

- not to bevy itself but creating a react like syntax macro for nodes with observe that function as event listeners
- Can't use right and left together on absolutely positioned elements
- left,right,bottom,top doesn't respect ContentBox

# Implementing Sound With Setting Menu For Options

```rust
mod VolumeSettings {
  pub struct VolumeSettings {
    SFX_vol: f32,
    SFX_on: bool,
    music_vol: f32,
    music_on: bool,
  };

  impl VolumeSettings {
    get_sfx() -> f32 {
      if SFX_on {
        SFX_vol;
      } else {
        0;
      }
    }
    // Same thing for get_music
    // add setters for each one
  }

  // plugin for inserting VolumeSettings as resource
}
```


## Settings in options

SFX volume slider
SFX toggle
Music volume slider
Music toggle

An x button to close the menu
A settings menu button the title screen that shows the settings screen

```rust
mod Settings {
  // a show custom command that puts it on top of everything else
  // checkbox and sliders that change the values in VolumeSettings
  // An x button that hides itself. Also activated with x.
}

```

Audio systems will simply read from the VolumeSettings struct modifying their playback settings.

## Implementing save indicator

Spawns itself, hides itself when leaving ingame, shows when entering ingame. Systems only run in game. follows the cursor.

Exposes two things for game_controls: set_save_indicator_green, set_save_indicator_red as systems. You can guess what they do.
