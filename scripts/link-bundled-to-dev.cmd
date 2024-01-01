@Echo off
cmd /c mklink /j "%COMMONPROGRAMFILES%\VST3\zmann-dev" "%~dp0target\bundled\"
echo Successfully linked 'bundled' to 'dev'.
