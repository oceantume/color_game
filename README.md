# color_game

A color-mixing puzzle game made for Bevy Jam #2.

## Game description

*Guess Hue?* is a color-mixing puzzle!

On every level, you get presented with a color that you have to replicate by mixing basic colors from the palette at the bottom. A complexity level indicates how many colors you will need to combine together to achieve that. By clicking on your palette, you add colors to your final mix and the color preview will be updated with what you made so far.

When you find the right color you gain a life (up to 3 lives). When you end up with the wrong color, you lose a life. If you run out of lives, you lose the game and have to start over.

The game currently has 25 levels with the color complexity increasing every 5 level.

Good luck!

## More information

https://oceantume.itch.io/guess-hue

Underneath, the game uses the mixbox library to generate paint-like color mixes from basic colors. It was an absolute pain to implement for the WASM build, but I'm happy with the result!

List of public assets used by the game:

https://opengameart.org/content/well-done
https://opengameart.org/content/10-retro-rpg-menu-sounds
https://opengameart.org/content/gui-sound-effects
https://www.dafont.com/edo-sz.font
https://www.dlf.pt/ddetail/TTRRbTb_white-brush-stroke-png-transparent-png/
