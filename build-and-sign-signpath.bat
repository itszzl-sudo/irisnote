@echo off
chcp 65001 >nul
echo ====================================
echo   IrisNote 构建和签名脚本
echo ====================================
echo.

REM 检查 Signpath 配置
if "%SIGNPATH_API_TOKEN%"=="" (
    echo ⚠️ Signpath 未配置，将使用自签名
    echo.
    echo 要使用 Signpath 签名，请设置环境变量：
    echo   SIGNPATH_API_TOKEN
    echo   SIGNPATH_ORG_ID
    echo.
    set USE_SIGNPATH=0
) else (
    echo ✅ Signpath 已配置
    set USE_SIGNPATH=1
)

echo.
echo [步骤 1/3] 构建项目...
echo.

cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo × 构建失败！
    pause
    exit /b 1
)

echo.
echo ✅ 构建成功！
echo.

if "%USE_SIGNPATH%"=="1" (
    echo [步骤 2/3] 使用 Signpath 签名...
    echo.
    
    REM 检查 Signpath CLI 是否安装
    where signpath >nul 2>&1
    if %ERRORLEVEL% NEQ 0 (
        echo ⚠️ Signpath CLI 未安装
        echo 请访问 https://signpath.io/docs/cli 安装
        echo.
        echo 或者等待 GitHub Actions 自动签名
        goto :packaging
    )
    
    REM 使用 Signpath 签名
    signpath sign --org-id %SIGNPATH_ORG_ID% --project irisnote --policy release-signing target\release\irisnote.exe
    
    if %ERRORLEVEL% EQU 0 (
        echo ✅ Signpath 签名成功！
    ) else (
        echo × Signpath 签名失败
        echo 将使用未签名版本
    )
) else (
    echo [步骤 2/3] 创建自签名证书...
    echo.
    
    powershell -Command "$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=IrisNote' -CertStoreLocation Cert:\CurrentUser\My; $path = Join-Path $env:USERPROFILE 'irisnote.cer'; Export-Certificate -Cert $cert -FilePath $path; Write-Host '证书已导出到: ' $path"
    
    echo.
    echo 正在签名可执行文件...
    
    powershell -Command "$cert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -like '*IrisNote*' } | Select-Object -First 1; $exe = Join-Path $env:CD 'target\release\irisnote.exe'; if (Test-Path $exe) { Set-AuthenticodeSignature -FilePath $exe -Certificate $cert; Write-Host '✅ 签名成功！' } else { Write-Host '× 可执行文件不存在' }"
)

:packaging
echo.
echo [步骤 3/3] 创建发布包...
echo.

REM 获取版本号
for /f "tokens=*" %%i in ('cargo pkgid --quiet 2^>nul ^| findstr /r ".*#.*"') do set PKGID=%%i
for /f "tokens=2 delims=#" %%i in ("%PKGID%") do set VERSION=%%i
if "%VERSION%"=="" set VERSION=0.1.0

set ZIP_NAME=irisnote-v%VERSION%-windows-x64.zip

REM 创建 ZIP 包
powershell -Command "Compress-Archive -Path 'target\release\irisnote.exe', 'README.md', 'README_CN.md', 'LICENSE', 'NOTICE' -DestinationPath '%ZIP_NAME%' -Force"

if exist "%ZIP_NAME%" (
    echo ✅ 发布包已创建: %ZIP_NAME%
    echo.
    echo 文件位置: %CD%\%ZIP_NAME%
    echo.
    for %%I in (%ZIP_NAME%) do echo 文件大小: %%~zI 字节
) else (
    echo × 创建发布包失败
)

echo.
echo ====================================
echo   构建完成！
echo ====================================
echo.
echo 可执行文件: target\release\irisnote.exe
echo 发布包: %ZIP_NAME%
echo.

if "%USE_SIGNPATH%"=="0" (
    echo 💡 提示: 配置 Signpath 可获得更专业的签名
    echo    参考: SIGNPATH_SETUP.md
    echo.
)

pause
