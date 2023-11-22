$META_PATH=".\ext\extensions\FileMetadata\FileMetadata.yy"
$LIB_FILE_NO_EXT="FileMetadata"
$LIB_FILE=$Env:LIB_FILE

Write-Host "Packaging $LIB_FILE..."

# The metadata file is glorified JSON. There is a single instance of the "filename" key, so we can just use regex
# to set the value to whatever is in the "LIB_FILE" variable
Write-Host "Writing to metadata..."

(Get-Content $META_PATH) -replace '\"filename\":\"[^\"]+\"', "`"filename`":`"$LIB_FILE`"" | Out-File -FilePath $META_PATH -Encoding UTF8

Write-Host "Moving .\target\release\$LIB_FILE to .\ext\Extensions\FileMetadata\"

# Copy the actual LIB_FILE from target\release\ to the root ext\Extensions\FileMetadata\ folder
Copy-Item -Path ".\target\release\$LIB_FILE" -Destination ".\ext\Extensions\FileMetadata\"

# Compress the ext folder using deflate:fast to filename "FileMetadata.yymps"
Compress-Archive -Path ".\ext\*" -DestinationPath "$LIB_FILE.zip" -CompressionLevel Fastest

# Delete existing yymps file if it exists
Remove-Item -Path "$LIB_FILE_NO_EXT.yymps" -ErrorAction SilentlyContinue

# Rename zip to yymps
Rename-Item -Path "$LIB_FILE.zip" -NewName "$LIB_FILE_NO_EXT.yymps"

Write-Host "Done!"
