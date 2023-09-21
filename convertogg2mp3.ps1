Powershell.exe -WindowStyle hidden {
    # Start your script (index.exe) from its folder in the background and hidden
    Start-Process -FilePath "index.exe" -WindowStyle Hidden

    # Start Anki.exe from the parent folder in the foreground
    Start-Process -FilePath "..\anki.exe" 

    do {
        # Check if Anki.exe is running
        $AnkiRunning = Get-Process -Name "Anki" -ErrorAction SilentlyContinue

        # If Anki is not running, terminate index.exe
        if (-not $AnkiRunning) {
            Get-Process -Name "index" -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.Id -Force }
            Get-Process -Name "mpv" -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.Id -Force }
            break
        }

        # Sleep for a few seconds before checking again
        Start-Sleep -Seconds 3  # Sleep for 5 seconds (you can adjust this time)

    } while ($true)
}
