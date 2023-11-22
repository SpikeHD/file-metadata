META_PATH=./ext/extensions/FileMetadata/FileMetadata.yy
LIB_NAME_NO_EXT=FileMetadata

echo "Packaging ${LIB_FILE}..."

# The metadata file is glorified JSON. There is a single instance of the "filename" key, so we can just use regex
# to set the value to whatever is in the "LIB_NAME" variable
sed -i -E "s/\"filename\":\"[^\"]+\"/\"filename\":\"${LIB_FILE}\"/g" ${META_PATH}

# copy the actual LIB_FILE from target/release/ to the root ext/Extensions/FileMetadata/ folder
cp ./target/release/${LIB_FILE} ./ext/extensions/FileMetadata/

# Compress the ext folder using deflate:fast to filename "FileMetadata.yymps"
zip -r -9 -Z deflate:fast ${LIB_NAME_NO_EXT}.yymps ./ext/*