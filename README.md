# Anki .OGG to .MP3 Converter

## Description
- Anki-ogg2mp3-converter is a simple script that automatically converts .**ogg media files** to .**mp3** in the Anki collection media folder, so that the audio is playable on IOS devices.
- It can convert the audio files as you create your cards && if you need to mass convert a deck already filled with ogg files, it will do it for you as soon as you launch anki using the **run.bat** script.
- Since Anki stores all of its media files in one folder, you dont need to specify any folders, it will just convert **all ogg files from all decks**, and you can replace the names using anki's built in regex.

## Installation

1. Download the **anki-convertogg2mp3.zip** from the **[Releases](https://github.com/aramrw/anki-ogg2mp3-converter/releases)** page.
2. **Drag and Drop** the **anki-convertogg2mp3.zip** into **Anki's main folder** *(where anki.exe is)* and extract the folder from the zip file. 

**The file hierarchy should look like this** 👇 
```
C:\Program Files
    └── Anki
        ├── anki-convertogg2mp3
        │   ├── convertogg2mp3.ps1
        │   ├── index.exe
        │   └── run.bat
        │
        ├── lib
        ├── anki.exe
        └── ...more anki files
```
3. **Run:** Inside the **anki-convertogg2mp3** folder, right click **"run.bat"** and press **"Create a Shortcut"**. Click **"Yes"** and now the **run.bat** shortcut should start up Anki.
4. - The program will now automatically convert all **ogg** files to **mp3** in Anki's **collections.media** folder located at **C:\Users\Your-Profile\AppData\Roaming\Anki2\User 1\collection.media** on startup.

## Usage
**To replace the .**ogg** file names to .**mp3** in Anki, first go to the deck you would like to convert**.
1. Press **Ctrl + Shift + A** to select **all cards** in the deck.
2. On the top left, click on **Notes** -> **Find and Replace** or press **Ctrl + Alt + F**.
3. In the **Find** box type **.ogg**. In the **Replace With** box type **.mp3**.

![anki_8p7dHm8BkX](https://github.com/aramrw/anki-ogg2mp3-converter/assets/106574385/b3e1b021-af5a-40c4-aa2f-c79474ebd669)

**Sync the changes, and now your cards audio should be playable on IOS**.
