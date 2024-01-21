@echo off
setlocal enabledelayedexpansion

rem List of files to download
set "files=Accordion.binv5 AltoSax.binv5 Bandoneon.binv5 Brass1.binv5 Brass2.binv5 BrassEnsemble.binv5 Cello.binv5 ChurchOrgan.binv5 Clarinet.binv5 ElecOrgan1.binv5 ElecOrgan2.binv5 ElecOrgan3.binv5 ElecOrgan4.binv5 Flute.binv5 FrenchHorn1.binv5 FrenchHorn2.binv5 Harmonica.binv5 Harp.binv5 Oboe.binv5 Piccolo.binv5 PipeOrgan.binv5 Recorder.binv5 ReedOrgan.binv5 SopranoSax.binv5 Soundtrack.binv5 Strings1.binv5 Strings2.binv5 Strings3.binv5 SynPad1.binv5 SynPad2.binv5 SynPad3.binv5 TenorSax.binv5 Trumpet.binv5 Tuba.binv5 Violin.binv5"

rem Set the output folder
set "outputFolder=samples\Toybox_c1200"

rem Create the output folder if it doesn't exist
if not exist "%outputFolder%" mkdir "%outputFolder%"

rem Download each file
for %%i in (%files%) do (
    set "file=%%i"
    set "url=https://zmann-bucket.onrender.com/artifacts/Toybox_c1200/!file!"
    echo Downloading !file!...
    powershell -Command "& { Invoke-WebRequest -Uri '!url!' -OutFile '%outputFolder%\!file!' }"
    echo.
)

echo All files downloaded successfully.

endlocal
