# raspberry-pi-arduino-serial-tauri

Project is an example with Tauri (`VueJS` + `Vuetify`) as Desktop app to communicate with an Arduino in Serial.  

---

---

# installation

### requirements

|            |           |
| ---------- | --------- |
| rust       | `1.74.1`  |
| nodejs     | `>= 20.0` |
| typescript | `>= 5.0`  |
| vue-cli    | `>= 5.0`  |

### others

Please check Tauri documentation for additional packages: https://tauri.app/

For `aarch64` compilation, additional packages are also required: https://tauri.app/v1/guides/building/linux#cross-compiling-tauri-applications-for-arm-based-devices

### setup

Default serial port used is `/dev/ttyACM0`.  
This can be changed in `main.rs`.

### arduino

Upload the script from `arduino/arduino.ino` to an Arduino board.

---

# commands

### development

For full stack:

```sh
npm run tauri dev
```

Client only:

```sh
npm run serve
```

### build

View [Tauri building](https://tauri.app/v1/guides/building/linux#cross-compiling-tauri-applications-for-arm-based-devices).

`build` will be output in `src-tauri/release/bundle/${APP_NAME}.${EXTENSION}`.

```sh
# default
npm run tauri build

# `aarch64` (Raspberry PI 64-bits):
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/
npm run build:arm
```

---

---

# how to

The default app will render 2 buttons to toggle a LED (on / off) via commands given through USB Serial to Arduino.

---

---

# documentation and links

- [Tauri](https://tauri.app/)
- [VueJS 3](https://vuejs.org/)
- [Vuetify 3](https://vuetifyjs.com/en/)
