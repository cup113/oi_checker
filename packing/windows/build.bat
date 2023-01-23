@echo off
call cargo build --release
copy /Y target\release\oi_checker.exe oi_checker.exe
copy oi_checker.exe packing\windows\oi-checker.exe
call Compil32 /cc packing\windows\setup.iss
del packing\windows\oi-checker.exe
