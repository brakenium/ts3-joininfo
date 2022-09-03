$ErrorActionPreference = "stop"

$targets = @(
    @{
        target = "x86_64-pc-windows-msvc"
        dll_suffix = "win32"
        WSL = $false
    },
    @{
        target = "x86_64-unknown-linux-gnu"
        dll_suffix = "linux_x86"
        WSL = $true
    }
)

$ts3_package_dir = "./ts3_package"
$ts3_plugin_dll_dir = "$($ts3_package_dir)/plugins"
$plugin_name = "ts3joininfo"
$ts3_plugin_output = ".\$($plugin_name).ts3_plugin"

Remove-Item "$($ts3_plugin_dll_dir)/*.dll"
foreach ($target in $targets) {
    if ($target.WSL) {
        wsl.exe -d debian cargo build --release --target $target.target
    }
    else {
        cargo build --release --target $target.target
    }
    Copy-Item "./target/$($target.target)/release/$($plugin_name).dll" "$($ts3_plugin_dll_dir)/$($plugin_name)_$($target.dll_suffix).dll"
}
7z.exe a -tzip ts3joininfo.ts3_plugin .\ts3_package\*
Start-Process $ts3_plugin_output