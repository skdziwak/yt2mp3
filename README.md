# yt2mp3
## Description

yt2mp3 is my first rust project. It downloads videos from YouTube and converts them to mp3.<br>
It also allows customizing mp3 metadata using sed.

## Help
```text
yt2mp3 1.0

Szymon Dziwak <skdziwak@gmail.com>

Downloads mp3 files from YouTube using youtube-dl. Allows customizing mp3 metadata.

USAGE:
    yt2mp3 [OPTIONS]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

OPTIONS:
    -a, --album <album>          Sed expression for evaluating mp3 album. Input format:
                                 'ID__CHANNEL__TITLE' [default: "s/^.+$/NO ALBUM/"]
    -p, --playlist <playlist>    YouTube playlist link
    -r, --artist <artist>        Sed expression for evaluating mp3 artist. Input format:
                                 'ID__CHANNEL__TITLE' [default: s/^.+__(.+)__.+$/\1/]
    -t, --title <title>          Sed expression for evaluating mp3 title. Input format:
                                 'ID__CHANNEL__TITLE' [default: s/^.+__.+__(.+)$/\1/]
    -v, --video <video>          YouTube video link
```

## Example usage
Downloading playlist of songs named like `[ARTIST] - [SONG_NAME] ([EXTRA_INFO])`
```shell
yt2mp3 -t 's/^.+?__.+?__(.+?) - (.+) \(.+$/\2/' -r 's/^.+?__.+?__(.+?) - (.+) \(.+$/\1/' -p [PLALIST_URL] 
```
Output metadata:<br>
Title: [SONG_NAME]<br>
Artist: [ARTIST]<br>
Album: NO_ALBUM