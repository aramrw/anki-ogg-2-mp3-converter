## Anki .OGG to .MP3 Converter

### Description

- Anki-ogg-2-mp3 converter is a simple script that automatically converts .**ogg media files** to .**mp3** in the Anki collection.media folder, so that the audio is playable on iOS devices.
- Since Anki stores all of its media files in one folder, you only need to specify the collection.media folder once, and it will convert **all ogg files from all decks**.
  [Then you can replace the names using anki's built in regex](#usage).
- Converts files on startup and once more when Anki closes for low memory overhead.

### Installation

[Virus Total Scan](https://www.virustotal.com/gui/file/071a6e8f9bc974adba19eed1dc622fdb44b0ab0db0ab32a17e3c3d399c3c774e?nocache=1)

1. Download `convert-ogg-mp3.exe` from the [Releases](https://github.com/aramrw/anki-ogg-2-mp3-converter/releases) page.
2. Simply drag and drop the .exe into `Anki's main folder` _(where anki.exe is)_.

**The file hierarchy should look like something this** 👇

```
C:\Program Files\Users\ExampleUser\AppData\Local\Programs\
    └── Anki
        ├── convert-ogg-mp3.exe
        ├── lib
        ├── anki.exe
        └── ...more anki files
```

3. Right click `convert-ogg-mp3.exe`, click `Create a Shortcut`, and put the shorcut wherever you want.
4. Click the shortcut, and a terminal window should prompt you for your `collection.media folder`. It is usually located inside `C:\Users\Exampleuser\AppData\Roaming\Anki2\User 1\collection.media`
5. Copy the path as is, and paste it into the terminal.

**The shortcut will now start up Anki and convert all .ogg files to .mp3**.

### Usage

You need to replace .ogg file names to .mp3 in Anki, so first go to the deck you would like to convert.

1. On the top left, click on `Notes` -> `Find and Replace` / or press `Ctrl + Alt + F`.
2. In the `Find` box type -> `.ogg` & in the `Replace With` box type -> `.mp3`

**Sync the changes, and now your cards audio should be playable on IOS**.
