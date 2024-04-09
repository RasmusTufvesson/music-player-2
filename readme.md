# Music Player 2
This is a small music player made in rust using rodio and eframe. The reason this is called Music Player *2* is that I tried to make this before but it just didn't work.

## How to use
All you have to do is create a `config.json` file that looks something like this:

```json
{
    "song_folder": "path/to/song/folder"
}
```

The actual ui of the program is pretty easy to figure out, there are two tabs, "Controls" and "Songs". In the controls tab you will find the general controls like pausing and skipping songs. Meanwhile in the songs tab you there is a list of the songs playing where you can skip to any song or shuffle all of them.