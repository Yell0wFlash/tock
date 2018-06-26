//! Capsule for analog sensors.
//!
//! This capsule provides the sensor HIL interfaces for sensors which only need
//! an ADC.
//!
//! It includes support for analog light sensors and analog temperature sensors.

use kernel::common::cells::OptionalCell;
use kernel::hil;
use kernel::ReturnCode;

/// The type of the sensor implies how the raw ADC reading should be converted
/// to a light value.
pub enum AnalogLightSensorType {
    LightDependentResistor,
}

pub struct AnalogLightSensor<'a, A: hil::adc::Adc> {
    adc: &'a A,
    channel: &'a <A as hil::adc::Adc>::Channel,
    sensor_type: AnalogLightSensorType,
    client: OptionalCell<&'a hil::sensors::AmbientLightClient>,
}

impl<A: hil::adc::Adc> AnalogLightSensor<'a, A> {
    pub fn new(adc: &'a A, channel: &'a <A as hil::adc::Adc>::Channel, sensor_type: AnalogLightSensorType) -> AnalogLightSensor<'a, A> {
        AnalogLightSensor {
            adc: adc,
            channel: channel,
            sensor_type: sensor_type,
            client: OptionalCell::empty(),
        }
    }
}

/// Callbacks from the ADC driver
impl<A: hil::adc::Adc> hil::adc::Client for AnalogLightSensor<'a, A> {
    fn sample_ready(&self, sample: u16) {
        // TODO: calculate the actual light reading.
        let measurement: usize = match self.sensor_type {
            AnalogLightSensorType::LightDependentResistor => sample as usize
        };
        self.client.map(|client| client.callback(measurement));
    }
}

impl<A: hil::adc::Adc> hil::sensors::AmbientLight for AnalogLightSensor<'a, A> {
    fn set_client(&self, client: &'static hil::sensors::AmbientLightClient) {
        self.client.set(client);
    }

    fn read_light_intensity(&self) -> ReturnCode {
        self.adc.sample(self.channel)
    }
}


/// The type of the sensor implies how the raw ADC reading should be converted
/// to a temperature value.
pub enum AnalogTemperatureSensorType {
    MicrochipMcp9700,
}

pub struct AnalogTemperatureSensor<'a, A: hil::adc::Adc> {
    adc: &'a A,
    channel: &'a <A as hil::adc::Adc>::Channel,
    sensor_type: AnalogTemperatureSensorType,
    client: OptionalCell<&'a hil::sensors::TemperatureClient>,
}

impl<A: hil::adc::Adc> AnalogTemperatureSensor<'a, A> {
    pub fn new(adc: &'a A, channel: &'a <A as hil::adc::Adc>::Channel, sensor_type: AnalogLightSensorType) -> AnalogLightSensor<'a, A> {
        AnalogLightSensor {
            adc: adc,
            channel: channel,
            sensor_type: sensor_type,
            client: OptionalCell::empty(),
        }
    }
}

/// Callbacks from the ADC driver
impl<A: hil::adc::Adc> hil::adc::Client for AnalogTemperatureSensor<'a, A> {
    fn sample_ready(&self, sample: u16) {
        // TODO: calculate the actual temperature reading.
        let measurement: usize = match self.sensor_type {
            // 𝑉out = 500𝑚𝑉 + 10𝑚𝑉/C ∗ 𝑇A
            AnalogTemperatureSensorType::MicrochipMcp9700 => sample as usize
        };
        self.client.map(|client| client.callback(measurement));
    }
}

impl<A: hil::adc::Adc> hil::sensors::TemperatureDriver for AnalogTemperatureSensor<'a, A> {
    fn set_client(&self, client: &'static hil::sensors::TemperatureClient) {
        self.client.set(client);
    }

    fn read_temperature(&self) -> ReturnCode {
        self.adc.sample(self.channel)
    }
}
