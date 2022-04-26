$ErrorActionPreference = "stop"

$target = "x86_64-pc-windows-msvc"
$ts3_package_dir = "./ts3_package"
$ts3_plugin_dll_dir = "$($ts3_package_dir)/plugins"
$plugin_name = "ts3joininfo"
$ts3_plugin_output = ".\$($plugin_name).ts3_plugin"

cargo build --release --target $target
Remove-Item "$($ts3_plugin_dll_dir)/*.dll"
Copy-Item "./target/$($target)/release/$($plugin_name).dll" "$($ts3_plugin_dll_dir)/$($plugin_name)_win32.dll"
Copy-Item "./target/$target/release/$($plugin_name).dll" "$($ts3_plugin_dll_dir)/$($plugin_name)_win64.dll"
7za.exe a -tzip ts3joininfo.ts3_plugin .\ts3_package\*
Start-Process $ts3_plugin_output