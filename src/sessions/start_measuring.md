# Start Measuring

# ❗️❗️❗️ defmt update

Last week, [defmt v0.1.0 to crates.io][defmt-crates] was released on crates.io. We prepared a handy [guide] on migrating your project from the github version of demft to the crates.io version. New projects based on the app-template will automatically use the crates.io version from now on.



After making sure, the communication is set up correctly by reading and logging the version number of the firmware, we'll start making measurements. 

An example of this implementation can be found here: [11_scd_30_measure.rs](https://github.com/knurling-rs/knurling-sessions-20q4/blob/main/src/bin/11_scd_30_measure.rs).

✅ Go to secion 1.4.1 of the [Interface Description]. 

What are the message components we have to provide, so that continous measuring is triggered?

<details>
    <summary>Answer</summary>

    0x00 Command byte
    0x10 Command byte
    0x00 Argument: Ambient Air Pressure
    0x00 Argument: Ambient Air Pressure
    0x81 CRC byte

    The start and stop sign are automatically provided by the write method.
</details>


This Message does not only contain a command, but also an argument which allows to set a value for ambient air pressure. 


# The Role of Ambient Air Pressure

Together with temperature, air pressure determines how many gas molecules can be found in a defined space. In an open system, the number of molecules rises when pressure increases and falls when pressure decreases. The Sensor's output unit for CO<sub>2</sub> is ppm, parts per million, which means of one million particles (atoms or molecules) the air contains as a whole, a certain number are Carbondioxode molecules. To be able to calculate that fraction, air pressure is needed. 

If a very accurate sensor reading is neccesary, the value for ambient air pressure should come from another sensor. When building an air quality monitor for work and school rooms, hardcoding a value is sufficient. The standart air pressure at sea-level is 1013.25 mbar, check you local weather station for a value if you live on higher altitudes. 

For this tutorial we use the current value from Berlin, which is 1020 mbar. 

# Start Continuous Measurement

✅ Go to `src/scd30/mod.rs`. In the `impl SCD30` block add a new function that takes `&mut self` as and `preassure: u16 `as arguments and returns a `Result` type with the variants `()` and `Error`.

Inside the function, define a mutable array for 5 `u8` bytes, as this is the length of the message we will send. Leave the argument bytes and the crc byte as `0x00`.

```rust
pub fn start_continuous_measurement(&mut self, pressure: u16) -> Result<(), Error> {
    let mut command: [u8; 5] = [0x00, 0x10, 0x00, 0x00, 0x00];
    // ...
    Ok(())
}
```

Next, we fill in the argument into the command. The sensor communication works in big endian byte order. 

✅ Convert the `u16` value into big-endian bytes. Assign the values contained in the returned slice to their respective positions in the command. 

```rust
let argument_bytes = &pressure.to_be_bytes();

command[2] = argument_bytes[0];
command[3] = argument_bytes[1];
```

# Calculating the CRC byte

If we send messages that are longer then two bytes, we need to send CRC bytes for verification after every two bytes. They need to be calculated from the argument bytes.

✅ Go to `cargo.toml` and add the following dependency:

```rust
crc_all = "0.2.0"
```

✅ Go back to `src/scd30/mod.rs` and bring the module into scope:

```rust
use crc_all::Crc;
```
✅ Check the documentation of [crc_all]. What arguments does the instance method `Crc::<u8>::new()` require?
Go to the [Interface Description] of the sensor, section 1.1.3 and check if you can fill in all the arguments. 

<details>
    <summary>Answer</summary>

    |arguments|information|
    |-|-|
    |poly: u8|0x31|
    |width: uszise|8|
    |init: u8|0xff|
    |xorout: u8|0x00|
    |reflect: bool|false|

</details>


✅ Inside `pub fn start_continuous_measurement()`, instantiate a new crc byte, with your gathered information. The variable needs to be mutable. Update the crc byte with the pressure value and assign the byte to its position in the command array. Send the command to the sensor. 

```rust
let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);

crc.update(&pressure.to_be_bytes());
command[4] = crc.finish();

self.0.write(DEFAULT_ADDRESS, &command)?;
```

✅ Go to your program file. In `fn main()` set the ambient air pressure and start measuring! 

```rust
// substitute 1020_u16 with your local value
let pressure = 1020_u16;


// ...
sensor.start_continuous_measurement(pressure).unwrap();

loop {
    ///...
}
```

✅ Run the program. you should see a blinking led.

After powering up, the sensor takes about 2 seconds until data is ready to be read. Besides just providing values, the sensor is also able to provide the information, if data is ready yet or not. 

✅ In `src/scd30/mod.rs`, implement the data_ready method. Check the interface description for the command and length of the read buffer. 

<details>
    <summary>Answer</summary>

    ```rust
    pub fn data_ready(&mut self) -> Result<bool, Error> {
    let command: [u8; 2] = [0x02, 0x02];
    let mut rd_buffer = [0u8; 3];

    self.0.write(DEFAULT_ADDRESS, &command).unwrap();
    self.0.read(DEFAULT_ADDRESS, &mut rd_buffer).unwrap();

    Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
    }
    ```
   
</details>


✅ In your program file, before the blinking loop, open a new loop, that constantly reads the sensor if data is ready. Add a log statement that prints "Data ready." once the method returns `true`. Then the loop breaks.

```rust
loop {
    if sensor.data_ready().unwrap() {
        defmt::info!("Data ready.");
        break
    }
```

✅ Run your program. You should see the log output "Data ready".


## Reading and logging sensor data

The sensor returns three values, one for Carbondioxide concentration, one for temperature and one for humidity. In section 1.5 in the [Interface Description] find the number type the sensor uses for the data. 

<details>
    <summary>Answer</summary>

    The values the sensor returns are float numbers in big-endian format.
    
</details>

✅ Go to `src/scd30/mod.rs`. Add a new `struct` definition, with a field for each value. 

```rust
pub struct SensorData {
    pub co2: f32,
    pub temperature: f32,
    pub humidity: f32,
}

pub const DEFAULT_ADDRESS: u8 = 0x61;
pub struct SCD30<T: Instance>(Twim<T>);

impl<T> SCD30<T>
where
    T: Instance,
{
    // ...
}
 ```   

✅ Inside the `impl SCD30` block, add a new method: 

```rust
pub fn read_measurement(&mut self) -> Result<SensorData, Error> {
    // ...
    Ok(data)
}
```

* Check the [Interface Description] for the command and length of the read buffer. 
* Make an instance of `SensorData`.
* Convert the relevant bytes into `f32` values. Check the std documentation for conversion of big endian bytes to f32.
* return the data.


<details>
    <summary>Answer</summary>

```rust
pub fn read_measurement(&mut self) -> Result<SensorData, Error> {
    let command: [u8; 2] = [0x03, 0x00];
    let mut rd_buffer = [0u8; 18];

    self.0.write(DEFAULT_ADDRESS, &command).unwrap();
    self.0.read(DEFAULT_ADDRESS, &mut rd_buffer).unwrap();

    let data = SensorData {
        co2: f32::from_bits(u32::from_be_bytes([
            rd_buffer[0],
            rd_buffer[1],
            rd_buffer[3],
            rd_buffer[4],
        ])),
        temperature: f32::from_bits(u32::from_be_bytes([
            rd_buffer[6],
            rd_buffer[7],
            rd_buffer[9],
            rd_buffer[10],
        ])),
        humidity: f32::from_bits(u32::from_be_bytes([
            rd_buffer[12],
            rd_buffer[13],
            rd_buffer[15],
            rd_buffer[16],
        ])),
    };
    Ok(data)
}
```
    
</details>


✅ In your program file, inside the blinking loop, call the method and add the values and their unit to the log. You can format using `\n` and `\r` for new lines.

```rust
loop {
    let result = sensor.get_measurement().unwrap();

    let co2 = result.co2;
    let temp = result.temperature;
    let humidity = result.humidity;

    defmt::info!("CO2 {:?} ppm \r\nTemperature {:?} C \r\nHumidity {:?} % \r\n\r\n",
        co2, temp, humidity
    );

    // blinking leds
}
```

✅ Run the program, you should see the three values in the log output. 


## Things to do on you own:

* The factory calibration of the sensor is pretty good, but you can still read up on how to calibrate the sensor and implement the necessary methods. 
* Implement the altitude compensation method and use it instead of pressure compensation. 
* Calculate absolute humidity from the relative humidiy value you can get from the sensor
* Calculate the dewpoint
* Implement the remaining methods.


[Interface Description]: https://www.sensirion.com/fileadmin/user_upload/customers/sensirion/Dokumente/9.5_CO2/Sensirion_CO2_Sensors_SCD30_Interface_Description.pdf

[crc_all]: https://docs.rs/crc_all/0.2.0/crc_all/
[guide]: https://defmt.ferrous-systems.com/migration.html  
[defmt-crates]: https://crates.io/crates/defmt

