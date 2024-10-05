/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2024 Suyashtnt (derived from Pierre Constantineau)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 */

#ifndef _WOAGBOARD_LEFT_H
#define _WOAGBOARD_LEFT_H

#define _PINNUM(port, pin) ((port) * 32 + (pin))

/*------------------------------------------------------------------*/
/* LED
 *------------------------------------------------------------------*/
#define LEDS_NUMBER 1
#define LED_PRIMARY_PIN _PINNUM(0, 28) // Yellow top of the board
#define LED_STATE_ON 0

/*------------------------------------------------------------------*/
/* BUTTON
 *------------------------------------------------------------------*/
#define BUTTONS_NUMBER 2        // note: no buttons connected by default
#define BUTTON_1 _PINNUM(0, 18) // unusable: RESET
#define BUTTON_2 _PINNUM(0, 3)  // not connected
#define BUTTON_PULL NRF_GPIO_PIN_PULLUP

/*------------------------------------------------------------------*/
/* UART (only used by nRF52832)
 *------------------------------------------------------------------*/
#define RX_PIN_NUMBER 8
#define TX_PIN_NUMBER 6
#define CTS_PIN_NUMBER 5
#define RTS_PIN_NUMBER 7
#define HWFC false // leaving it false to make GPIO available

//--------------------------------------------------------------------+
// BLE OTA
//--------------------------------------------------------------------+
#define BLEDIS_MANUFACTURER "Suyashtnt"
#define BLEDIS_MODEL "Woagboard"

//--------------------------------------------------------------------+
// USB
//--------------------------------------------------------------------+
// TODO: get one from https://github.com/openmoko/openmoko-usb-oui. The one below is for development only.

#define USB_DESC_VID 0x1209
#define USB_DESC_UF2_PID 0x0001
#define USB_DESC_CDC_ONLY_PID 0x0001

#define UF2_PRODUCT_NAME "Woagboard (Left Half)"
#define UF2_VOLUME_LABEL "WOAGLEFT"
#define UF2_BOARD_ID "nRF52840-woagboard-left"
#define UF2_INDEX_URL "https://github.com/Suyashtnt/woagboard"

#endif // _WOAGBOARD_LEFT_H
