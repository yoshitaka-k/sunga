#!/bin/sh

case "$(uname -s)" in
Darwin)
  rm -rf ~/Library/Application\ Support/Sunga
  ;;
Linux)
  data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
  rm -rf "$data_home"/sunga
  ;;
MINGW*|MSYS*|CYGWIN*)
  rm -rf "$APPDATA"/Sunga
  ;;
*)
  echo "unsupported OS: $(uname -s)" >&2
  exit 1
  ;;
esac

exit
