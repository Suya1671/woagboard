# Woagboard Firmware: RMK

RMK is a feature-rich and easy-to-use keyboard firmware, which has VIAL support and is incredibly power efficient.

The default layout is the wobbler's personal layout, using dust and a bunch of fancy features.
I'll eventually probably maybe make the default QWERTY and less opinionated.

## uf2 support

If you’re using the Adafruit_nRF52_Bootloader (Recommended for the Woagboard), you’re in luck! This bootloader supports the .uf2 firmware format, which eliminates the need for a debugging probe to flash your firmware. RMK uses the `cargo-make` tool to generate .uf2 firmware, with the generation process defined in the `Makefile.toml`.

Follow these steps to generate and flash the .uf2 firmware with RMK:

1. Get `cargo-make` tool:
   ```shell
   cargo install --force cargo-make
   ```
2. Compile RMK and generates .uf2 firmware:
   ```shell
   cargo make uf2 --release
   ```
3. Flash
   - For each half:
    - Put your half into bootloader mode. This can be done by quickly double tapping the reset button.
    - A USB drive will appear on your computer under the name "WOAGBOARD".
    - Drag and drop the generated .uf2 firmware file onto the USB drive. The RMK firmware will be automatically flashed onto the Woagboard.

### Additional notes

RMK defaults to USB-priority mode if a USB cable is connected. After flashing, remember to disconnect the USB cable, or [switch to BLE-priority mode](https://rmk.rs/docs/features/wireless.html#multiple-profile-support) by pressing User7(Switch Output) key.
