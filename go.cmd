@echo off

setlocal

cargo build
if errorlevel 1 exit /b 1

cl.exe /c main.cpp /Z7
if errorlevel 1 exit /b 1

link ^
ws2_32.lib ^
ntdll.lib ^
advapi32.lib ^
userenv.lib ^
bcrypt.lib ^
/incremental:no ^
/dll ^
/def:all.def ^
/debug ^
/out:main.dll ^
main.obj ^
target\debug\blue.lib ^
target\debug\green.lib
