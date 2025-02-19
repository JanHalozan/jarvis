# Documentation

Raspberry PI 5 startup script for fixing audio:

```shell
#!/bin/bash

pactl set-default-sink alsa_output.usb-C-Media_Electronics_Inc._USB_Audio_Device-00.analog-steroe
pactl unload-module module-udev-detect && pactl load-module module-udev-detect
```
