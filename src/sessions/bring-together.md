# Bringing it all Together
## Using the LED as Temperature Indicator

You have learned the following:
* Lighting and wiring RGB LEDs.
* Using a temperature sensor.
* Implementing User Input

Build a program that indicates temperatures around your personal comfort temperature with different light colors. 

What is the temperature where you feel most comfortable?
Define a spectrum around that temperature spanning 3 Degrees (°C). Temperatures below that interval are too cold, temperatures above are too hot. The middle degree is your comfort zone, around it are acceptable values. Assign a signal color for each zone. Feel free to adapt the ranges. 

✅ Integrate this behavior of the LED into the last program. 

❗️ Refactor the way the `struct LEDState` is defined, if you want to rely on that code. It is bound to specific pins, but can be rewritten so that it can take any gpio, just like `struct Button`. 



