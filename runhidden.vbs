Set objShell = CreateObject("WScript.Shell")
Set objWMIService = GetObject("winmgmts:\\.\root\cimv2")

' Start your script (index.exe) from its folder
objShell.Run "index.exe", 0, False

' Start Anki.exe from the parent folder
objShell.Run "..\Anki.exe", 1, False

Do
    ' Query the list of running processes
    Set colProcesses = objWMIService.ExecQuery("Select * from Win32_Process")

    ' Flag to check if Anki is running
    AnkiRunning = False

    ' Check if Anki.exe is among the running processes
    For Each objProcess In colProcesses
        If LCase(objProcess.Name) = "anki.exe" Then
            AnkiRunning = True
            Exit For
        End If
    Next

    ' If Anki is not running, terminate index.exe
    If Not AnkiRunning Then
        objShell.Run "taskkill /f /im index.exe", 0, False
        Exit Do
    End If

    ' Sleep for a few seconds before checking again
    WScript.Sleep 5000  ' Sleep for 5 seconds (you can adjust this time)

Loop
