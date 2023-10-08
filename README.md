# mac-mic-volume

A simple CLI tool to set the microphone volume to the maximum in MacOS.

## Setup as just the cli tool

<https://github.com/amiyzku/max-mic-volume/releases>

## Setup as deamon

```bash
sudo mv max-mic-volume /usr/local/bin/

sudo curl -o /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist https://raw.githubusercontent.com/amiyzku/max-mic-volume/master/com.github.amiyzku.max-mic-volume.plist
sudo chown root:wheel /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist
sudo chmod 644 /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist
```
