@Echo off

echo Please run this script as an administrator.
pause

cmd /c mklink /j "%COMMONPROGRAMFILES%\VST3\zmann-dev" "%~dp0target\bundled\"

echo Successfully linked 'bundled' to 'dev'.
