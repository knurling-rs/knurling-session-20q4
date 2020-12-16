# Glossary

## HAL

HAL is short for Hardware Abstraction Layer. A HAL is a set of routines that provide interfaces for programs to access hardware resources.

## GPIO

GPIO is short for General Purpose Input Output. GPIOs are programmable, digital or sometimes analogue signal pins that can be used as interfaces to other systems or devices. 

## Pin Configurations
### Floating
A floating pin is neither connected VCC nor ground. The voltage will match the residual voltage.

### Push-Pull-Output

A pin that is configured as push–pull output can switch between high and low voltage.

### Open-Drain-Output
Open Drain outputs switch between "disconnected" and "connected to ground".


### Pull-Up-Input
A pin that is configured as pull-up input is set to VCC, as long as it is not overwritten by an external source. This setting prevents the pin from floating, which can cause noise in the system. 

# Protocols

## I2C 

The I2C protocol has two signal lines, one for data (SDA) and one for the clock signal (SCL). An I2C transaction consists of one or more messages. Each message begins with a start symbol. A message is either *write* or *read*, indicated by the next bit in the row. Following this comes the actual message in form of bytes. The message ends with a stop symbol. 
The clock signal rises and falls in a specified frequency. 

Many devices can be connected to the same I2C bus and messages can be send to a particular device by specifying its I2C address.
