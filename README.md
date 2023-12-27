# raspberry-pi-arduino-serial-tauri

Project is an example with Tauri (`VueJS` + `Vuetify`) as Desktop app to communicate with an Arduino in Serial.  
Project aims to attempt development under a single language, `Rust`.

---

---

# directory stucture

```sh
/
  /src # vuejs client
  /src-arduino # arduino program in rust to upload
  /src-tauri # rust backend

```

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

For Arduino compilation and build from `Rust`, follow this tutorial: https://blog.logrocket.com/complete-guide-running-rust-arduino/  
To consume Arduino `C++` librairies (from Arduino **Library Manger**), follow this tutorial: https://dev.to/kgrech/five-simple-steps-to-use-any-arduino-c-library-in-a-rust-project-1k78

### setup

Default serial port used is `/dev/ttyACM0`.  
This can be changed in `main.rs`.

---

---

# arduino

Upload the script from `arduino/arduino.ino` to an Arduino board.

### board schema

In `arduino/arduino.ino`, the **GPIO** pin `#2` is used.

![image](https://github.com/JimJ92120/raspberry-pi-arduino-serial-tauri/assets/57893611/79483a27-91bd-4938-8844-e3441934528d)

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

### tauri build

View [Tauri building](https://tauri.app/v1/guides/building/linux#cross-compiling-tauri-applications-for-arm-based-devices).

`build` will be output in `src-tauri/release/bundle/${APP_NAME}.${EXTENSION}`.

```sh
# default
npm run tauri build

# `aarch64` (Raspberry PI 64-bits):
export PKG_CONFIG_SYSROOT_DIR=/usr/aarch64-linux-gnu/
npm run build:arm
```

### arduino build

The `src-arduino` will get compiled and upload to the Arduino board.  
`avrdude` is used to compile for Arduino boards.

```sh
npm run upload:arduino
```

---

---

# how to

The default app will render 2 buttons to toggle a LED (on / off) via commands given through USB Serial to Arduino.

![image](https://github.com/JimJ92120/raspberry-pi-arduino-serial-tauri/assets/57893611/87fbb84f-6bfa-44c1-a398-7b9d3687802f)

### serial logs

Some "logs" are printed in serial monitor (e.g VSCode):  
![image](https://github.com/JimJ92120/raspberry-pi-arduino-serial-tauri/assets/57893611/d4b145c3-94f1-4dba-a361-f2d883e824f5)

---

---

# documentation and links

- [Tauri](https://tauri.app/)
- [VueJS 3](https://vuejs.org/)
- [Vuetify 3](https://vuetifyjs.com/en/)
- [Rust for Arduino tutorial](https://blog.logrocket.com/complete-guide-running-rust-arduino/)
- [Arduino Libraries in Rust](https://dev.to/kgrech/five-simple-steps-to-use-any-arduino-c-library-in-a-rust-project-1k78)
