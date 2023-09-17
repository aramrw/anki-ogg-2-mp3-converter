import * as path from 'path';
import * as fs from 'fs';
import * as ffmpeg from 'fluent-ffmpeg';
import * as os from 'os';
import { stdout } from 'process';

const ankiMediaPath = path.join(os.homedir(), 'AppData', 'Roaming', 'Anki2', 'User 1', 'collection.media');
const outputFolder = path.join(ankiMediaPath, 'mp3'); // Folder to store the converted mp3 files

fs.mkdirSync(outputFolder, { recursive: true }); // Create the output folder if it doesn't exist

async function convertFile(oggFile): Promise<void> {
    return new Promise((resolve, reject) => {
        const oggFilePath = path.join(ankiMediaPath, oggFile);
        const mp3File = oggFile.replace('.ogg', '.mp3');
        const mp3FilePath = path.join(outputFolder, mp3File);

        ffmpeg()
            .input(oggFilePath) // Set the input file
            .output(mp3FilePath) // Set the output file
            .on('end', function () {
                resolve();
            })
            .on('error', function (err) {
                console.log(`Error converting ${oggFile}:`, err);
                reject(err);
            })
            .run();
    });
}

(async () => {

    let counter = 0;
    const files = await fs.promises.readdir(ankiMediaPath);
    const oggFiles = files.filter(file => file.endsWith('.ogg'));

    console.clear();
    console.log(`Starting conversion of ${oggFiles.length} files.\n`);

    for (const oggFile of oggFiles) {
        try {
            await convertFile(oggFile);
            counter++;
            stdout.write(`Converted ${counter} of ${oggFiles.length}\r`);
        } catch (error) {
            // Handle errors if needed
        }
    }

    console.clear();
    console.log('Conversion finished');
})();
