echo Downloading latest foreground-cron to ~/.bin
mkdir -p ~/.bin &&
rm -f ~/.bin/foreground-cron &&
curl --silent https://api.github.com/repos/arve0/foreground-cron/releases/latest \
  | grep browser_download_url \
  | cut -d '"' -f 4 \
  | wget -qP ~/.bin/ -i - &&
chmod +x ~/.bin/foreground-cron &&
if ! [ -x "$(command -v foreground-cron)" ]; then
  echo 'export PATH="$HOME/.bin:$PATH"' >> ~/.bash_profile;
  echo "Added 'export PATH=\"\$HOME/.bin:\$PATH\"' to ~/.bash_profile";
  echo "Run 'source ~/.bash_profile' to enable new path or restart your shell"
fi
