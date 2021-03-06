#!/bin/bash

FOLDER="."
FILES='.+\.\(md\|rs\|\sh\|toml\|txt\|yml\)'

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo -n ">>> Seaching for files without a trailing newline..."

# Check any text file
FOUND=0
for FILE in $(find $FOLDER -regex $FILES); do
    # Ignore files that are ignored by git
    git check-ignore -q $FILE && continue

    # Search for trailing newline
    LASTLINE=$(tail -n 1 $FILE; echo x)
    LASTLINE=${LASTLINE%x}
    if [ "${LASTLINE: -1}" != $'\n' ]; then
        [ $FOUND == 0 ] && echo -e "\tError."
        echo -e "Found:\t$FILE"
        FOUND=1
    fi
done
test $FOUND == 0
echo -e "\tDone."
