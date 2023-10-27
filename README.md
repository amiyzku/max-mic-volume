# max-mic-volume

A simple CLI tool to set the microphone volume to the maximum in MacOS.
However, it does not maximize when muted.

![behavior](images/behavior.png)

## Usage

```bash
$ ./max-mic-volume --help
A simple CLI tool to set the microphone volume to the maximum in MacOS

Usage: max-mic-volume [OPTIONS]

Options:
  -p, --polling-interval-ms <POLLING_INTERVAL_MS>  [default: 3000]
  -h, --help                                       Print help
  -V, --version                                    Print version

```

## Setup as CLI tool

<https://github.com/amiyzku/max-mic-volume/releases>

## Setup as deamon

```bash
sudo mv max-mic-volume /usr/local/bin/
sudo chmod 755 /usr/local/bin/max-mic-volume

sudo curl -o /Library/LaunchAgents/com.github.amiyzku.max-mic-volume.plist https://raw.githubusercontent.com/amiyzku/max-mic-volume/master/com.github.amiyzku.max-mic-volume.plist
sudo chmod 644 /Library/LaunchAgents/com.github.amiyzku.max-mic-volume.plist

launchctl load /Library/LaunchAgents/com.github.amiyzku.max-mic-volume.plist
```
