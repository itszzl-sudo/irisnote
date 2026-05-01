@echo off
echo 正在构建 IrisNote...

cargo build --release

if %ERRORLEVEL% EQU 0 (
    echo 构建成功！
    echo.
    echo 正在创建自签名证书...
    
    powershell -Command "$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=IrisNote' -CertStoreLocation Cert:\CurrentUser\My; $path = Join-Path $env:USERPROFILE 'irisnote.cer'; Export-Certificate -Cert $cert -FilePath $path; Write-Host '证书已导出到: ' $path"
    
    echo.
    echo 正在签名可执行文件...
    
    powershell -Command "$cert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -like '*IrisNote*' } | Select-Object -First 1; $exe = Join-Path $env:CD 'target\release\irisnote.exe'; if (Test-Path $exe) { Set-AuthenticodeSignature -FilePath $exe -Certificate $cert; Write-Host '签名成功！' } else { Write-Host '可执行文件不存在' }"
    
    echo.
    echo 完成！可执行文件位于: target\release\irisnote.exe
) else (
    echo 构建失败！
    pause
    exit /b 1
)

pause
