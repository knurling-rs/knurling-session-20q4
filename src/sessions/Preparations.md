# Preparations

This chapter contains informations about knurling-sessions, the standard hardware and an installation guide.

## Standard Hardware

Knurling Sessions 2020 Q4 assumes you're using the [nRF52840 Development Kit (DK)](https://www.nordicsemi.com/Software-and-Tools/Development-Kits/nRF52840-DK) as your development board. You can also use any other board supported by [`probe-run`](https://github.com/knurling-rs/probe-run), but you may have to make some modifications to the provided instructions on your own.

The DK needs to be connected to your PC with *1* micro-USB cables (make sure they're data cables and not just for charging). The chapter on [checking your hardware](./hardware.md) will provide you with more details.

You'll need a few more peripherals and parts - all in all:

### Bill of Materials
#### Block 1: Getting started with embedded Rust

![nrf52840-dk](../images/nrf52840-dk.png)

* 1x nrf52840-DK (or other nrf52XXX boards)
    * [Digikey](https://www.digikey.com/short/zf03q4)
* 1x RGB-LED (or single colored LED(s) and/or on-board LEDs)
    * [Digikey](https://www.digikey.com/short/zf03mq)
* 3x 220 Ohm Resistors
    * [Digikey](https://www.digikey.com/short/zf0398)
* 1x Potentiometer
    * [Digikey](https://www.digikey.com/short/zf0355)
* 1x Breadboard
    * [Digikey](https://www.digikey.com/short/zf03rh)
* 1x Jumper wires (40 wires) - Pin to Pin
    * [Digikey](https://www.digikey.com/short/zf03fw)
* 1x Jumper wires (40 wires) - Pin to Receptacle
    * [Digikey](https://www.digikey.com/short/zf0328)
* 1x Jumper wires (40 wires) - Receptacle to Receptacle
    * [Digikey](https://www.digikey.com/short/zf032z)

#### Block 2: Adding the CO2 Sensor

![scd30](../images/scd30.png)

**Note**: Soldering is required for this step to connect the headers of the Sensirion gas sensor. Alternatively, you may be able to use "Hook Probes", such as the ones offered by [Adafruit](https://www.adafruit.com/product/238), instead of soldering. These are also often available in bulk from websites such as Aliexpress.

* 1x Sensirion SCD30 CO2 Sensor (or other air quality sensor)
    * [Digikey](https://www.digikey.com/short/zf0q30)
* 1x Pin Headers (40 piece)
    * [Digikey](https://www.digikey.com/short/zf0qjc)
* 1x Piezo buzzer
    * [Digikey](https://www.digikey.com/short/zf0q5p)

#### Block 3: Embedded Graphics

![waveshare display](../images/waveshare_4_2.jpg)

**Note**: Soldering is required for this step to connect the headers of the Waveshare display. Alternatively, you may be able to use "Hook Probes", such as the ones offered by [Adafruit](https://www.adafruit.com/product/238), instead of soldering. These are also often available in bulk from websites such as Aliexpress.

* 1x Waveshare 4.2 inch b&w ePaper Display
    * [Waveshare](https://www.waveshare.com/product/displays/e-paper/epaper-2/4.2inch-e-paper-module.htm)



