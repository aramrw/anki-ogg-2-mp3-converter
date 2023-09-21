const chokidar = require('chokidar');
const { execSync } = require('child_process');

const miningFolder = 'C:\\Users\\arami\\Desktop\\Mining';

const watcher = chokidar.watch(miningFolder, {
    ignoreInitial: true,
    ignored: /(^|[/\\])\../, // Ignore dotfiles
    persistent: true, // Keep the process running
});

watcher
    .on('ready', () => console.log('Ready to watch...'))
    .on('change', (path) => {
        const fileName = path.split('\\')[5];
        stringPath = path.toString();

        if (fileName.endsWith('.mp3')) {
            setTimeout(() => {
                console.log('File: ' + fileName + ' has been added.');
                copyToClipboard(stringPath);
            }, 2000);
        }
    });

function copyToClipboard(text) {
    try {
        execSync(`echo ${text} | clip`, { stdio: 'ignore' });
        console.log('Text copied to clipboard successfully.');
    } catch (error) {
        console.error('Error copying text to clipboard:', error);
    }
}
