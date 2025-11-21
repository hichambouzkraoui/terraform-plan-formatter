# Terraform Plan Formatter Windows installer
param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:USERPROFILE\.local\bin"
)

$ErrorActionPreference = "Stop"

$repo = "example/terraform-plan-formatter"
$target = "x86_64-pc-windows-gnu"

Write-Host "Installing terraform-plan-formatter for Windows..."

# Create install directory
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Create temp directory
$tempDir = New-TemporaryFile | ForEach-Object { Remove-Item $_; New-Item -ItemType Directory -Path $_ }

try {
    # Download URL
    if ($Version -eq "latest") {
        $downloadUrl = "https://github.com/$repo/releases/latest/download/tfplan-$Version-$target.zip"
    } else {
        $downloadUrl = "https://github.com/$repo/releases/download/v$Version/tfplan-$Version-$target.zip"
    }

    $zipPath = Join-Path $tempDir "tfplan.zip"
    
    Write-Host "Downloading from $downloadUrl..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile $zipPath
    
    # Extract
    Expand-Archive -Path $zipPath -DestinationPath $tempDir
    
    # Install binary
    $binaryPath = Join-Path $tempDir "tfplan-$Version-$target" "tfplan.exe"
    $installPath = Join-Path $InstallDir "tfplan.exe"
    
    Copy-Item $binaryPath $installPath -Force
    
    Write-Host "✓ terraform-plan-formatter installed to $installPath"
    
    # Add to PATH if not already there
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
    if ($userPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("PATH", "$userPath;$InstallDir", "User")
        Write-Host "✓ Added $InstallDir to PATH (restart terminal to use)"
    }
    
    Write-Host "Run 'tfplan --help' to get started!"
    
} finally {
    # Cleanup
    Remove-Item $tempDir -Recurse -Force
}