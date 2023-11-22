META_PATH=./ext/extensions/FileMetadata/FileMetadata.yy
LIB_NAME_NO_EXT=FileMetadata

echo "Packaging ${LIB_FILE}..."

# The metadata file is glorified JSON. There is a single instance of the "filename" key, so we can just use regex
# to set the value to whatever is in the "LIB_NAME" variable
UNAME=$(uname -s)

if [[ ${UNAME} == "Darwin" ]]; then
  sed -i '' -E "s%\"filename\":\"[^\"]+\"%\"filename\":\"${LIB_FILE}\"%g" ${META_PATH}  
else
  sed -i -r "s%\"filename\":\"[^\"]+\"%\"filename\":\"${LIB_FILE}\"%g" ${META_PATH}
fi

# copy the actual LIB_FILE from target/release/ to the root ext/Extensions/FileMetadata/ folder
cp ./target/release/${LIB_FILE} ./ext/extensions/FileMetadata/

# Compress the ext folder using deflate:fast to filename "FileMetadata.yymps"
zip -r -1 ${LIB_NAME_NO_EXT}.yymps ./ext/*