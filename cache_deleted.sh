#!/bin/sh

case "$(uname -s)" in
Darwin)
  rm -rf ~/Library/Application\ Support/Resizer
  ;;
Linux)
  data_home="${XDG_DATA_HOME:-$HOME/.local/share}"
  rm -rf "$data_home"/resizer
  ;;
MINGW*|MSYS*|CYGWIN*)
  rm -rf "$APPDATA"/Resizer
  ;;
*)
  echo "unsupported OS: $(uname -s)" >&2
  exit 1
  ;;
esac

exit
