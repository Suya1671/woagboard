# Woagboard: Bisexual egg edition
_name chosen by popular vote._

![Woagboard](./render/Render.png)

**ROUND 3** This is the third hardware revision of the Woagboard keyboard. It's a 5x3 split ergonomic keyboard with 3 thumb keys on each half (36 keys total), inspired by the [TOTEM](https://github.com/GEIGEIGEIST/TOTEM).

I made this keyboard for myself, and to learn PCB design in the process. With the newest iteration, I've gotten into case design (Actually properly learning CAD for once), way more annoying part selection processes, and overall a chaotic time.

# Features

![Woagboard with the case opened](./render/Render-Open.png)

- 36 Keys
  - 5x3 matrix, with 3 thumb keys on each half
  - Splayed, column-staggered, ergonomic design
- Wireless, with generous space to fit a battery
  - 400mAh should last you a month
- On-board MCU
  - Nordic nRF52840, for BLE and USB-C connectivity
- Chaotic silkscreen printing
- Optional nice!view display mounting point
- RMK and ZMK firmware support

## The case
![Woagboard Case](./render/Render-Angled.png)

The case is designed to be 3D printed, with a focus on being thin, minimalistic, and clean. It features cutouts for all the necessary ports and buttons, and has a dedicated space for the nice!view display on the left half.
It also features cutouts for [Tenting Feet](https://www.aliexpress.com/item/1005005605228469.html) that make the keyboard way more comfy to use.

## The PCB

![Woagboard PCB](./render/Render-PCB.png)

Silly, chaotic, yet somehow functional. The PCB is as small as the layout allows it to be, featuring an on-board Ebyte E73-2G4M08S1C module for wireless connectivity (Powered by the Nordic nRF52840), a BQ24075RGT battery charger (Which allows you to charge the keyboard while it's running. Such a revolutionary feature), and a 32.768kHz crystal for deep sleep functionality.

It also has really chaotic silkscreening. We stay silly.

![Woagboard PCB Silkscreen](./render/Render-PCB-Silkscreen.png)

### Schematic

![Woagboard Schematic](./pictures/woagboard-schematic.svg)
:3

# BOM
This is the Bill of Materials (BOM) for 1 Woagboard (2 halves). This is why certain quanitites are doubled, as the board is two halves using the same PCB and components.
| Component                                      | Designator on PCB/Usage                            | Supplier     | Supplier URL                                            | Quantity                   |
| ---------------------------------------------- | -------------------------------------------------- | ------------ | ------------------------------------------------------- | -------------------------- |
| Woagboard PCB                                  | Placed in case                                     | JLCPCB       | See pcb/jlcpcb for production files                     | 2                          |
| Ebyte E73-2G4M08S1C MCU*                       | U1                                                 | LCSC         | <https://lcsc.com/product-detail/C356849.html>          | 2                          |
| TI BQ24075RGT Battery Charger                  | U2                                                 | LCSC         | <https://lcsc.com/product-detail/C15464.html>           | 2                          |
| Onsemi SRV05-4 ESD Protection                  | U3                                                 | LCSC         | <https://lcsc.com/product-detail/C2836319.html>         | 2                          |
| Korean Hroparts Elec TYPE-C-31-M-12            | J1                                                 | LCSC         | <https://lcsc.com/product-detail/C165948.html>          | 2                          |
| 1x6 Vertical Pin header                        | J2                                                 | LCSC         | <https://lcsc.com/product-detail/C706869.html>          | 2                          |
| Panasonic EVQ-P7C01P push button               | PWR1,RST1                                          | LCSC         | <https://lcsc.com/product-detail/C388883.html>          | 4                          |
| Yellow LED                                     | LED1                                               | LCSC         | <https://lcsc.com/product-detail/C2296.html>            | 2                          |
| Green LED                                      | LED2                                               | LCSC         | <https://lcsc.com/product-detail/C2297.html>            | 2                          |
| 4.7uF Capacitor                                | C2,C3,C4,C5                                        | LCSC         | <https://lcsc.com/product-detail/C23733.html>           | 8                          |
| 1uF Capacitor                                  | C1,C6                                              | LCSC         | <https://lcsc.com/product-detail/C52923.html>           | 4                          |
| 18pF Capacitor                                 | C7,C8                                              | LCSC         | <https://lcsc.com/product-detail/C1549.html>            | 4                          |
| 47kΩ Resistor                                  | R4                                                 | LCSC         | <https://lcsc.com/product-detail/C25792.html>           | 2                          |
| 10kΩ Resistor                                  | R7                                                 | LCSC         | <https://lcsc.com/product-detail/C25744.html>           | 2                          |
| 5.1kΩ Resistor                                 | R1,R2                                              | LCSC         | <https://lcsc.com/product-detail/C25905.html>           | 4                          |
| 3.9kΩ Resistor                                 | R6                                                 | LCSC         | <https://lcsc.com/product-detail/C23018.html>           | 2                          |
| 270Ω Resistor                                  | R3,R5,R8                                           | LCSC         | <https://lcsc.com/product-detail/C25867.html>           | 6                          |
| 750 mA fuse                                    | F1                                                 | LCSC         | <https://lcsc.com/product-detail/C2687878.html>         | 2                          |
| 32.768kHz Crystal                              | Y1                                                 | LCSC         | <https://lcsc.com/product-detail/C32346.html>           | 2                          |
| Murata Electronics BLM15PX121SN1D Ferrite Bead | L1                                                 | LCSC         | <https://lcsc.com/product-detail/C88970.html>           | 2                          |
| 10uH Inductor                                  | L2                                                 | LCSC         | <https://lcsc.com/product-detail/C223249.html>          | 2                          |
| Kailh PG1350 Choc Hotswap Sockets              | SW1-SW18                                           | AliExpress** | <https://www.aliexpress.com/item/1005004290562374.html> | 30 x 2 (36 used)           |
| Woagboard Case                                 | Surrounds PCB. Connected Via M2 screws             | 3D Printed   | See case for models                                     | 1x Left, 1x Right          |
| M2x10 Countersunk Screws                       | H1 (Mounting Point)                                | JKM          | <https://jkm.co.za/product/m2x10-csk-galv-m-screw-bos/> | 2                          |
| M2x12 Countersunk Screws                       | H2, H3 (Mounting Points)                           | JKM          | <https://jkm.co.za/product/m2x12-csk-galv-m-screw-bos/> | 4                          |
| M2 Brass Heat inserts                          | H1, H2, H3 (Mounting Points)                       | AliExpress   | <https://www.aliexpress.com/item/32890237459.html>      | 50 x 1 (6 M2 inserts used) |
| Laptop Tenting Feet                            | Attached to case                                   | AliExpress** | <https://www.aliexpress.com/item/1005005605228469.html> | 2 x 2                      |
| Choc switches                                  | SW1-SW18 (Connected via hotswap)                   | AliExpress   | <https://www.aliexpress.com/item/1005005883472162.html> | 30 x 2 (36 used)           |
| Choc keycaps                                   | SW1-SW18 (Connected via switch)                    | AliExpress   | <https://www.aliexpress.com/item/1005004558099208.html> | 50 x 1 (36 used)           |
| nice!view                                      | J4 (Wired up with wires. nice!view placed in case) | Typeractive  | <https://typeractive.xyz/products/nice-view>            | 1                          |

Notes:

- \* Alternativelty, the Ebyte E73-2G4M08S1CX can be used for a UFL connector, requiring an external antenna. Both options annoyingly got moved to standard assembly only while I was making this design, increasing costs dramatically (~$60 -> ~$100). I've contacted JLCPCB about this, and hopefully I can get it back on economic assembly for anybody else trying to make this board.
- ** Typeractive also has these. If you're in the US, might be better to buy from them instead.
- I already have switches and keycaps, so I don't need to purchase them.
- I am not purchasing the nice!view for the fact that I'm out of budget if I do. Thanks JLCPCB.

# Copyright notice
The Woagboard is not affiliated with New Blood Interactive, Tour De Pizza, or any other IPs referenced on the design.
I just like ULTRAKILL, Pizza Tower, etc.

- Keycap model taken from https://www.thingiverse.com/thing:4564253 (CC BY-NC)
- Keyswitch model taken from https://github.com/kiswitch/kiswitch
- nice!view model taken from https://www.printables.com/model/1114611-niceview-oled/files (CC BY-NC)
- Neocats from https://volpeon.ink/emojis/neocat/ (CC-BY-NC-SA-4.0)
- Neofox from https://volpeon.ink/emojis/neofox/ (CC-BY-NC-SA-4.0)
- Neodog from https://git.gay/moonrabbits/neodog/src/branch/main/neodog/neodog.png (CC-BY-NC-SA-4.0)
- Neorat from https://emoji-repo.absturztau.be/repo/neorat.zip (CC-BY-NC-SA-4.0)
- Neobread from https://github.com/olivvybee/emojis/blob/main/neobread/neobread.svg (CC-BY-NC-SA-4.0)
- Neobot from https://github.com/SymTrkl/emoji/blob/main/webres/neobot/neobot.png (CC-BY-NC-SA-4.0)
- Monadic Cat's mask from https://www.catmonad.xyz/pfp.html (CC-BY-SA-4.0)

The board's actual license is the CERN Open Hardware License v1.2. See the [license file](./license.md) for more information.
