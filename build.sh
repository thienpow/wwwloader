#!/bin/sh


SOURCE_FOLDER='./'
TARGET_FOLDER='~/wwwloader'
PROD_SERVER='alpine'
USER='root'

rsync -av $SOURCE_FOLDER $USER@$PROD_SERVER:$TARGET_FOLDER --exclude-from='.rsyncignore'
ssh $USER@$PROD_SERVER 'bash -s' < remote_build.sh

# copy back the results
rsync -av $USER@$PROD_SERVER:$TARGET_FOLDER/.build ./