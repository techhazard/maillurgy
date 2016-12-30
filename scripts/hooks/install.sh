#!/bin/bash

# bash voodoo to find absolute path of the directory this file is in without symlinks
# taken from stackoverflow, seems to work well
FIND_CONF_DIR="${BASH_SOURCE[0]}"
while [ -h "$FIND_CONF_DIR" ]; do # resolve $FIND_CONF_DIR until the file is no longer a symlink
  githooksdir="$( cd -P "$( dirname "$FIND_CONF_DIR" )" && pwd )"
  FIND_CONF_DIR="$(readlink "$FIND_CONF_DIR")"
  [[ $FIND_CONF_DIR != /* ]] && FIND_CONF_DIR="$DIR/$FIND_CONF_DIR" # if $FIND_CONF_DIR was a relative symlink, we need to resolve it relative to the path where the symlink file was located
done
githooksdir="$( cd -P "$( dirname "$FIND_CONF_DIR" )" && pwd )"


all_githooks='applypatch-msg commit-msg post-update pre-applypatch pre-commit pre-push pre-rebase prepare-commit-msg update'

used_githooks="$(cd $githooksdir; ls $all_githooks 2>/dev/null)"
git_root="$(git rev-parse --show-toplevel)"

cd $git_root
for hook in $used_githooks;
do
  ln -sf "${githooksdir}/${hook}" "${git_root}/.git/hooks/$hook"
done
