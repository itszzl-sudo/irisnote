#!/usr/bin/env pwsh
# IrisNote 智能提交脚本

param(
    [Parameter(Mandatory=$true, Position=0)]
    [string]$Message,
    
    [Parameter(Position=1)]
    [ValidateSet("patch", "minor", "major", "none")]
    [string]$Bump = "patch"
)

function Get-CurrentVersion {
    $cargoContent = Get-Content Cargo.toml -Raw
    if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
        return $matches[1]
    }
    return "0.0.0"
}

function Update-Version {
    param([string]$CurrentVersion, [string]$Type)
    
    $parts = $CurrentVersion.Split('.')
    $major = [int]$parts[0]
    $minor = [int]$parts[1]
    $patch = [int]$parts[2]
    
    switch ($Type) {
        "major" {
            $major++
            $minor = 0
            $patch = 0
        }
        "minor" {
            $minor++
            $patch = 0
        }
        "patch" {
            $patch++
        }
    }
    
    return "$major.$minor.$patch"
}

function Update-CargoToml {
    param([string]$NewVersion)
    
    $cargoContent = Get-Content Cargo.toml -Raw
    $newContent = $cargoContent -replace '(version\s*=\s*)"[^"]+"', "`$1`"$NewVersion`""
    Set-Content Cargo.toml -Value $newContent -NoNewline
}

# Stage all changes
Write-Host "📝 Staging changes..." -ForegroundColor Cyan
git add -A

# Check if there are changes to commit
$status = git status --porcelain
if ([string]::IsNullOrWhiteSpace($status)) {
    Write-Host "⚠️  No changes to commit" -ForegroundColor Yellow
    exit 0
}

# Bump version if requested
if ($Bump -ne "none") {
    $currentVersion = Get-CurrentVersion
    $newVersion = Update-Version -CurrentVersion $currentVersion -Type $Bump
    
    Write-Host "📦 Bumping version: $currentVersion → $newVersion" -ForegroundColor Yellow
    Update-CargoToml -NewVersion $newVersion
    
    git add Cargo.toml
}

# Commit with message
Write-Host "💾 Committing..." -ForegroundColor Cyan
git commit -m "$Message"

# Show result
$newVersion = Get-CurrentVersion
Write-Host ""
Write-Host "✅ Committed successfully" -ForegroundColor Green
Write-Host "   Version: v$newVersion" -ForegroundColor Cyan
Write-Host "   Message: $Message" -ForegroundColor White
Write-Host ""

# Ask to push
Write-Host "Push to remote? (y/n)" -ForegroundColor Yellow
$response = Read-Host
if ($response -eq 'y') {
    Write-Host "🚀 Pushing to origin..." -ForegroundColor Cyan
    git push origin master
    
    Write-Host ""
    Write-Host "✅ Pushed successfully" -ForegroundColor Green
    Write-Host "   View at: https://github.com/itszzl-sudo/irisnote" -ForegroundColor Cyan
}
