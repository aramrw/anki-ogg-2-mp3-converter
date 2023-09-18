import * as path from 'path';
import * as fs from 'fs';
import * as ffmpeg from 'fluent-ffmpeg';
import * as os from 'os';
import { stdout } from 'process';

const ankiMediaPath = path.join(os.homedir(), 'AppData', 'Roaming', 'Anki2', 'User 1', 'collection.media');
const outputFolder = path.join(ankiMediaPath, 'mp3'); // Folder to store the converted mp3 files

const debounceDelay = 5000; // 5 seconds
let debounceTimer;
let isWatching = false;

fs.mkdirSync(outputFolder, { recursive: true }); // Create the output folder if it doesn't exist

console.clear();
mainProgram();

async function mainProgram() {
    try {

        // Convert all .ogg files in the directory on startup

        try {
            const files = await fs.promises.readdir(ankiMediaPath);
            const oggFiles = files.filter(file => file.endsWith('.ogg'));

            if (oggFiles.length > 0) {
                console.clear();
                console.log(`Found ${oggFiles.length} .ogg files and starting conversion.\n`);
                await startConversion();
            } else {
                console.log('No .ogg files found in the directory.\n');
            }
        } catch (error) {
            console.error('Error reading directory:', error);
        }

        // Watch the directory for changes

        console.clear();
        console.log(`Watching ${ankiMediaPath} for changes...\n`);
        const watcher = fs.watch(ankiMediaPath, { recursive: true }, async (eventType, filename) => {
            if (eventType === 'change' && filename.endsWith('.ogg')) {
                console.clear();
                console.log(`File ${filename} changed.`);

                // Clear any existing debounce timer
                clearTimeout(debounceTimer);

                // Start a new debounce timer
                debounceTimer = setTimeout(async () => {
                    // Check if the flag is still true
                    watcher.close();
                    console.log('Starting conversion...\n')
                    await startConversion();

                }, debounceDelay);
            }
        });

        watcher.on('error', (error) => {
            console.error(`Watcher error: ${error}`)
            process.exit();
        })



    } catch (error) {
        console.error('Error in main program:', error);

    }

};

async function startConversion() {
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

    let counter = 0;
    const files = await fs.promises.readdir(ankiMediaPath);
    const oggFiles = files.filter(file => file.endsWith('.ogg'));
    let startDeleting = false;

    console.log(`Starting conversion of ${oggFiles.length} files.\n`);

    await Promise.all(oggFiles.map(async oggFile => {
        try {
            const mp3File = oggFile.replace('.ogg', '.mp3');
            const mp3FilePath = path.join(outputFolder, mp3File);

            if (files.includes(mp3File)) {
                console.log();
                console.log(`${mp3File} already exists, skipping conversion.\n`);
            } else {
                await convertFile(oggFile);
                counter++;
                stdout.write(`Converted ${counter} of ${oggFiles.length}\r`);
            }
        } catch (error) {
            console.error(`Error converting ${oggFile}:`, error);
        }
        startDeleting = true;
    }));

    console.clear();
    console.log('Conversion finished');
    console.log('Deleting .ogg files...\n');


    // Now, delete original .ogg files
    if (startDeleting) {
        for (const oggFile of oggFiles) {
            try {
                await fs.promises.unlink(path.join(ankiMediaPath, oggFile));
                console.clear();
                console.log(`Deleted ${oggFile}\r`);
            } catch (error) {
            }
        }

        console.log('\nFinished Conversion and Deletion.\n');
        await mainProgram();
    }

}