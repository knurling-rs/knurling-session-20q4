use crc_all::Crc;

use nrf52840_hal::twim::{Error, Instance, Twim};
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
    pub fn init(i2c2: Twim<T>) -> Self {
        SCD30(i2c2)
    }

    pub fn get_firmware_version(&mut self) -> Result<[u8; 2], Error> {
        let command: [u8; 2] = [0xd1, 0x00];
        let mut rd_buffer = [0u8; 2];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        let major = u8::from_be(rd_buffer[0]);
        let minor = u8::from_be(rd_buffer[1]);

        Ok([major, minor])
    }

    pub fn start_continuous_measurement(&mut self, pressure: u16) -> Result<(), Error> {
        
        // command bytes
        let mut command: [u8; 5] = [0x00, 0x10, 0x00, 0x00, 0x00];
        let argument_bytes = &pressure.to_be_bytes();

        command[2] = argument_bytes[0];
        command[3] = argument_bytes[1];

        let mut crc = Crc::<u8>::new(0x31, 8, 0xff, 0x00, false);

        defmt::info!("{:?}", command);

        crc.update(&pressure.to_be_bytes());
        command[4] = crc.finish();
        defmt::info!("{:?}", command);

        self.0.write(DEFAULT_ADDRESS, &command)?;

        Ok(())
    }

    pub fn data_ready(&mut self) -> Result<bool, Error> {
        let command: [u8; 2] = [0x02, 0x02];
        let mut rd_buffer = [0u8; 3];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

        Ok(u16::from_be_bytes([rd_buffer[0], rd_buffer[1]]) == 1)
    }

    pub fn read_measurement(&mut self) -> Result<SensorData, Error> {
        let command: [u8; 2] = [0x03, 0x00];
        let mut rd_buffer = [0u8; 18];

        self.0.write(DEFAULT_ADDRESS, &command)?;
        self.0.read(DEFAULT_ADDRESS, &mut rd_buffer)?;

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
}
