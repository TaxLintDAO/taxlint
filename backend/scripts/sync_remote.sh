#!/bin/bash
# 这个脚本没有处理conflict.需要自行手动 git merge. 然后在vscode里面选择合并内容


echo -----pull from remote--------
git pull origin master
echo ----------staus--------------
git status
echo ----------add .--------------
git add .   # add all files in the current directory and its subdirectories
echo ----------commit-------------
git commit -m "$1" # use the first argument as the commit message
echo ----------push---------------
git push -u  origin master #the push-operation need to config remote address.
# i didn`t check it for you in the script
echo ----------status-------------
git status
echo ----------done-------------------

