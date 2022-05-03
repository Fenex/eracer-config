e-Racer (Форсаж) is a video game for Microsoft Windows created by Rage Software in 2001.

The game is pretty old and unfortunately there is not support high resolution and widescreen in out of box. However, it is possible to add support this features. Here you can find utility that can add support widescreen and high resolutions (up to 1920x1200 16:10 tested).

If your `eracer.exe` is not running at all, try to set execution compatibility with `Windows XP SP3`.

## Change resolution of e-Racer via [`eracer-config`](https://github.com/Fenex/eracer-config/releases):

Run `eracer-config` without any arguments to see current settings of the game.
To change resolution run: `eracer-config --set-resolution 1920x1200`.
To change aspect ratio run: `eracer-config --set-aspect-ratio 16:10`.
You can change ratio & resolution together on same run, just passthrough both keys.

## Manually change resolution of e-Racer:

What do you need do if you want to add support manually (algorithm of the utility):

* Edit windows registry.
    Open the windows registry and find an entry: `HKEY_CURRENT_USER\SOFTWARE\Rage Games Ltd\eRacer`. There you can see two properties: `PREFERRED HEIGHT` and `PREFERRED WIDTH` that needs to be changed as you want.

* Patch the binary.
    The game does not apply correct aspect ratio. For example, if you set resolution as 1920x1080, then all in the game will be flatten (very-very fat cars). You have to patch `eracer.exe` file to fix that. To do this, open your `eracer.exe` in your favorite hex editor and goto address:
    * `0x0C7ED8` if you have RU version (`eracer.exe` is 1008KB)
    * `0x0CAEFC` if you have EN version (992KB)

You should see there sequence of three bytes: `0x3A, 0x46, 0x71`. You need replace its with:

| bytes | ratio
|----|----|
| `0x66 0x66 0x66` | 5:4
| `0x3B 0xDF 0x87` | 25:16
| `0x61 0xE0 0x89` | 16:10
| `0xBA 0x2C 0x8E` | 15:9
| `0xE3 0xA5 0x93` | 16:9
| `0x29 0x5C 0xAF` | 21:9

Ready!
