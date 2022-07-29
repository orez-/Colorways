# Colorways Level File Format

The first line of the file contains the a character representing the initial light color of the level (see below), optionally followed by a "scene tag" character for special-case rendering.

The remaining lines describe the level, laid out in a grid, where each character is a tile.
The meaning of each character is detailed below.

- `#` is an impassible wall
- `.` is an empty tile
- `a` is the player's starting position. There must be **exactly one** starting position on each level.
- `z` is an exit goal. There may be multiple exit goals; reaching any exit will complete the level.
- `~` is water. Blocks may be sunk in water, destroying both and leaving an empty tile.

In addition, there exist combination characters, which compose an entity type with a color.
The colors are:
- `k` - black
- `r` - red
- `g` - green
- `b` - blue
- `c` - cyan
- `y` - yellow
- `m` - magenta
- `w` - white

and the entity types are:
- ` ̂` - a toggle light switch. When stepped on, lights of this color are toggled.
- ` ̊` - a radio light switch. When stepped on, all lights are turned off and lights of this color are turned on.
- ` ̽` - a block. Blocks may be pushed, or walked through when in a matching colored light.
- a capital letter represents a lightbulb. Lightbulbs may emit light matching their color.

For example, `r̽` is a red block.
All non-ascii combinations of colors and entity types can be found below:
```
k̂r̂ĝb̂ĉŷm̂ŵ
k̊r̊g̊b̊c̊ẙm̊ẘ
k̽r̽g̽b̽c̽y̽m̽w̽
```
