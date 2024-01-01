@echo off
setlocal enabledelayedexpansion

rem List of files to download
set "files=Accordion.binv3 AltoSax.binv3 Bandoneon.binv3 Brass1.binv3 Brass2.binv3 BrassEnsemble.binv3 Cello.binv3 ChurchOrgan.binv3 Clarinet.binv3 ElecOrgan1.binv3 ElecOrgan2.binv3 ElecOrgan3.binv3 ElecOrgan4.binv3 Flute.binv3 FrenchHorn1.binv3 FrenchHorn2.binv3 Harmonica.binv3 Harp.binv3 Oboe.binv3 Piccolo.binv3 PipeOrgan.binv3 Recorder.binv3 ReedOrgan.binv3 SopranoSax.binv3 Soundtrack.binv3 Strings1.binv3 Strings2.binv3 Strings3.binv3 SynPad1.binv3 SynPad2.binv3 SynPad3.binv3 TenorSax.binv3 Trumpet.binv3 Tuba.binv3 Violin.binv3"

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
