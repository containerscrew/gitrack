#!/bin/sh

set -e

# Global vars
INSTALLATION_PATH="/usr/local/bin/"

happyexit(){
  echo ""
  echo "gitrack successfully installed! 🎉"
  echo ""
  echo "Now run: $ gitrack --help"
  echo ""
  exit 0
}

# Check OS
OS=$(uname -s)
arch=$(uname -m)
cli_arch=""
case $OS in
  Darwin)
    case $arch in
      x86_64)
        cli_arch=amd64
        ;;
      arm64)
        cli_arch=$arch
        ;;
      *)
        echo "There is no gitrack $OS support for $arch"
        exit 1
        ;;
    esac
    ;;
  Linux)
    case $arch in
      x86_64)
        cli_arch=amd64
        ;;
      armv8*)
        cli_arch=arm64
        ;;
      aarch64*)
        cli_arch=arm64
        ;;
      amd64|arm64)
        cli_arch=$arch
        ;;
      *)
        echo "There is no gitrack $OS support for $arch"
        exit 1
        ;;
    esac
    ;;
  *)
    echo "There is no gitrack $OS support for $arch"
    exit 1
    ;;
esac
OS=$(echo "$OS" | tr '[:upper:]' '[:lower:]')

download_release() {
  LATEST_VERSION=$(curl -s https://api.github.com/repos/containerscrew/gitrack/releases/latest | jq -r ".name")
  if [ -z "$1" ]; then VERSION=$LATEST_VERSION; else VERSION=$1; fi

  printf "\033[0;32m[info] - Downloading version: ${VERSION}/gitrack-${OS}-${cli_arch}.zip \033[0m\n"
  curl -L --fail --remote-name-all https://github.com/containerscrew/gitrack/releases/download/"${VERSION}"/gitrack-"${OS}"-"${cli_arch}".zip -o /tmp/gitrack.zip
  unzip -o /tmp/gitrack.zip -d /tmp/
}

install_binary(){
  if [ "$(id -u)" = 0 ]; then
      cp /tmp/gitrack $INSTALLATION_PATH
      chmod +x $INSTALLATION_PATH/gitrack
  else
      sudo cp /tmp/gitrack $INSTALLATION_PATH
      sudo chmod +x $INSTALLATION_PATH/gitrack
  fi
  rm -rf /tmp/gitrack*
  happyexit
}

# Function to display help text
usage() {
    echo "Usage: $0 [-v] [-h]"
    echo "Options:"
    echo "  -v           Select which version do you want to install."
    echo "  -h           Display the help message"
}

# Parse options using getopts
while getopts "v:h" option; do
    case "${option}" in
        v)  # Install specific version
            version=${OPTARG}
            download_release "$version"
            install_binary
            ;;
        h)  # Help option
            usage
            exit 0
            ;;
        \?) # Invalid option
            echo "Invalid option: -${OPTARG}"
            usage
            exit 1
            ;;
    esac
done

# If no flags, by default install latest version
if [ $# -eq 0 ]; then
    download_release
    install_binary
fi
