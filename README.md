# mac-mic-volume

A simple CLI tool to set the microphone volume to the maximum in MacOS.

![behavior](images/behavior.png)

## Setup as CLI tool

<https://github.com/amiyzku/max-mic-volume/releases>

## Setup as deamon

```bash
sudo mv max-mic-volume /usr/local/bin/

sudo curl -o /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist https://raw.githubusercontent.com/amiyzku/max-mic-volume/master/com.github.amiyzku.max-mic-volume.plist
sudo chown root:wheel /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist
sudo chmod 644 /Library/LaunchDaemons/com.github.amiyzku.max-mic-volume.plist
```
