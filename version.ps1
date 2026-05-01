#!/usr/bin/env pwsh
# IrisNote 版本管理脚本

param(
    [Parameter(Position=0)]
    [ValidateSet("patch", "minor", "major", "show", "tag")]
    [string]$Action = "patch"
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
    
    # Update workflow files if they reference version
    Get-ChildItem -Path .github\workflows -Filter "*.yml" -Recurse | ForEach-Object {
        $content = Get-Content $_.FullName -Raw
        if ($content -match 'IrisNote v\d+\.\d+\.\d+') {
            $newContent = $content -replace 'IrisNote v\d+\.\d+\.\d+', "IrisNote v$NewVersion"
            Set-Content $_.FullName -Value $newContent -NoNewline
        }
    }
}

function Create-Tag {
    param([string]$Version)
    
    $tagName = "v$Version"
    
    # Delete old tag if exists
    git tag -d $tagName 2>$null
    git push origin :refs/tags/$tagName 2>$null
    
    # Create new tag
    git tag -a $tagName -m "Release $tagName"
    
    Write-Host "✅ Created tag: $tagName" -ForegroundColor Green
}

switch ($Action) {
    "show" {
        $version = Get-CurrentVersion
        Write-Host "Current version: $version" -ForegroundColor Cyan
    }
    
    { $_ -in @("patch", "minor", "major") } {
        $currentVersion = Get-CurrentVersion
        $newVersion = Update-Version -CurrentVersion $currentVersion -Type $Action
        
        Write-Host "Version: $currentVersion → $newVersion" -ForegroundColor Yellow
        
        Update-CargoToml -NewVersion $newVersion
        
        Write-Host "✅ Updated Cargo.toml to version $newVersion" -ForegroundColor Green
        
        # Commit the version bump
        git add Cargo.toml .github/workflows/*.yml
        git commit -m "chore: bump version to $newVersion" --no-verify
        
        Write-Host "✅ Committed version bump" -ForegroundColor Green
    }
    
    "tag" {
        $version = Get-CurrentVersion
        Create-Tag -Version $version
        
        Write-Host "Push tag to trigger release? (y/n)" -ForegroundColor Yellow
        $response = Read-Host
        if ($response -eq 'y') {
            git push origin "v$version"
            Write-Host "✅ Pushed tag v$version" -ForegroundColor Green
        }
    }
}
