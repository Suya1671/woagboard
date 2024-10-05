rm Adafruit_nRF52_Bootloader/src/boards/left -r
rm Adafruit_nRF52_Bootloader/src/boards/right -r
cp left Adafruit_nRF52_Bootloader/src/boards/left -r
cp right Adafruit_nRF52_Bootloader/src/boards/right -r

cd Adafruit_nRF52_Bootloader
make BOARD=left all
make BOARD=right all

cd ..
mkdir -p build/left
mkdir -p build/right
cp Adafruit_nRF52_Bootloader/_build/build-left/left_bootloader* build/left
cp Adafruit_nRF52_Bootloader/_build/build-right/right_bootloader* build/right
