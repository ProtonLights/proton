@echo off

set openssl_download=http://slproweb.com/download/Win64OpenSSL-1_1_0f.exe
set openssl_cert_download=https://curl.haxx.se/ca/cacert.pem
set sfml_download=https://www.sfml-dev.org/files/SFML-2.4.2-windows-vc14-64-bit.zip
set csfml_download=https://www.sfml-dev.org/files/CSFML-2.4-windows-64-bit.zip

set script_dir=%~dp0
set zipjs_location=%script_dir%\zipjs.bat
set tmp_dir=%script_dir%temp_%RANDOM%

rem Administrator permissions needed (making directory in C drive)
rem Check permissions with net session command (https://stackoverflow.com/questions/4051883/batch-script-how-to-check-for-admin-rights)
net session >NUL 2>&1
if %ERRORLEVEL% NEQ 0 (
	echo This script needs administrator privileges ^(right click, run as administrator^)
	goto END
)

echo Creating ProtonDependencies directory
@rmdir /s /q C:\ProtonDependencies 2>NUL
@mkdir C:\ProtonDependencies
if %ERRORLEVEL% NEQ 0 (
	echo Failed to create ProtonDependencies directory. Aborting.
	goto END
)

echo Setting up temp directory
@mkdir %tmp_dir%
if %ERRORLEVEL% NEQ 0 (
	echo Failed to create ProtonDependencies directory. Aborting.
	goto END
)

echo -------------------------
echo OpenSSL
set openssl_exe=%tmp_dir%\OpenSSL.exe
set openssl_cert=%tmp_dir%\cacert.pem

echo Downloading OpenSSL 1.1.0f
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:openssl_download, $env:openssl_exe)"

echo Installing OpenSSL
@start %openssl_exe%
set /p waiting=Please go through the install wizard and press Enter when done. (P.S. don't donate at the end if you don't want to)

echo Downloading cacert file
@powershell -Command "[System.Net.ServicePointManager]::SecurityProtocol = 'Tls12';(New-Object Net.WebClient).DownloadFile($env:openssl_cert_download, $env:openssl_cert)"

echo Moving cacert file
@move %openssl_cert% C:\OpenSSL-Win64\certs\cacert.pem >NUL

echo Moving OpenSSL to ProtonDependencies
@move C:\OpenSSL-Win64 C:\ProtonDependencies\OpenSSL-Win64 >NUL

echo Setting OPENSSL_DIR
setx OPENSSL_DIR C:\ProtonDependencies\OpenSSL-Win64 >NUL

echo Setting SSL_CERT_FILE
setx SSL_CERT_FILE C:\ProtonDependencies\OpenSSL-Win64\certs\cacert.pem >NUL

echo -------------------------
:SFML
echo SFML

set sfml_zip=%tmp_dir%\SFML.zip
set sfml_extracted=%tmp_dir%\SFML

echo Downloading SFML 2.4.2
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:sfml_download, $env:sfml_zip)"

echo Extracting SFML
@call %zipjs_location% unzip -source %sfml_zip% -destination %sfml_extracted%

echo Copying SFML files
@move %sfml_extracted%\SFML-2.4.2 C:\ProtonDependencies\ >NUL

echo Setting SFML_HOME
@setx SFML_HOME C:\ProtonDependencies\SFML-2.4.2 >NUL

echo -------------------------
echo CSFML

set csfml_zip=%tmp_dir%\CSFML.zip
set csfml_extracted=%tmp_dir%\CSFML

echo Downloading CSFML 2.4
@powershell -Command "(New-Object Net.WebClient).DownloadFile($env:csfml_download, $env:csfml_zip)"

echo Extracting CSFML
@call %zipjs_location% unzip -source %csfml_zip% -destination %csfml_extracted%

echo Copying CSFML files
@move %csfml_extracted%\CSFML C:\ProtonDependencies\ >NUL

echo Setting CSFML_HOME
@setx CSFML_HOME C:\ProtonDependencies\CSFML >NUL

echo -------------------------
echo Appending PATH
@setx PATH "%PATH%;C:\ProtonDependencies\CSFML\bin;C:\ProtonDependencies\SFML-2.4.2\bin" >NUL

echo Removing temp directory
@rmdir /s /q %tmp_dir%

echo Setup finished! If there were no errors, you should now be able to compile and run proton, proton-cli, and proton-runner

:END
	echo Press any key to exit setup...
	pause >NUL
