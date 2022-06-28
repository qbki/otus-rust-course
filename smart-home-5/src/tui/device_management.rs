use super::utils::{get_input, get_number, wrong_command};
use crate::common::{Device, DeviceInterface, Report, SwitchStatusEnum};
use crate::smart_outlet::SmartOutlet;
use crate::smart_thermometer::SmartThermometer;
use std::io;

#[derive(PartialEq)]
enum OutletManagement {
    Exit,
    Greeting,
    WrongCommand,
    Report,
    On,
    Off,
    SetPower,
}

#[derive(PartialEq)]
enum ThermometerManagement {
    Exit,
    Greeting,
    WrongCommand,
    Report,
    SetTemperature,
}

#[derive(PartialEq)]
enum GenericManagement {
    Exit,
    Greeting,
    WrongCommand,
    Report,
}

fn outlet_management(writer: &mut dyn io::Write, device: &SmartOutlet) -> io::Result<()> {
    let mut state = OutletManagement::Greeting;

    while state != OutletManagement::Exit {
        match state {
            OutletManagement::Greeting => {
                writeln!(writer, "Outlet Management")?;
            }
            OutletManagement::Report => {
                writeln!(writer, "{}", device.report_to_string())?;
            }
            OutletManagement::On => {
                device.set_switch(SwitchStatusEnum::On);
            }
            OutletManagement::Off => {
                device.set_switch(SwitchStatusEnum::Off);
            }
            OutletManagement::SetPower => {
                writeln!(writer, "Enter power state:")?;
                let power = get_number(writer);
                if let Result::Ok(number) = power {
                    device.set_power(number);
                }
            }
            _ => {
                wrong_command(writer)?;
            }
        }

        writeln!(
            writer,
            "{}",
            [
                "1 - On",
                "2 - Off",
                "3 - Set power level",
                "4 - Report",
                "5 - Back",
            ]
            .join("\n")
        )?;

        state = match get_input() {
            Result::Ok(input) => match input.as_ref() {
                "1" => OutletManagement::On,
                "2" => OutletManagement::Off,
                "3" => OutletManagement::SetPower,
                "4" => OutletManagement::Report,
                "5" => OutletManagement::Exit,
                _ => OutletManagement::WrongCommand,
            },
            _ => OutletManagement::WrongCommand,
        };
    }

    Result::Ok(())
}

fn thermometer_management(writer: &mut dyn io::Write, device: &SmartThermometer) -> io::Result<()> {
    let mut state = ThermometerManagement::Greeting;

    while state != ThermometerManagement::Exit {
        match state {
            ThermometerManagement::Greeting => {
                writeln!(writer, "Thermometer management")?;
            }
            ThermometerManagement::Report => {
                writeln!(writer, "{}", device.report_to_string())?;
            }
            ThermometerManagement::SetTemperature => {
                writeln!(writer, "Enter temperature:")?;
                let power = get_number(writer);
                if let Result::Ok(number) = power {
                    device.set_temperature(number);
                }
            }
            _ => {
                wrong_command(writer)?;
            }
        }

        writeln!(
            writer,
            "{}",
            ["1 - Set temperature", "2 - Report", "3 - Back"].join("\n")
        )?;

        state = match get_input() {
            Result::Ok(input) => match input.as_ref() {
                "1" => ThermometerManagement::SetTemperature,
                "2" => ThermometerManagement::Report,
                "3" => ThermometerManagement::Exit,
                _ => ThermometerManagement::WrongCommand,
            },
            _ => ThermometerManagement::WrongCommand,
        };
    }

    Result::Ok(())
}

fn generic_management(writer: &mut dyn io::Write, device: &dyn DeviceInterface) -> io::Result<()> {
    let mut state = GenericManagement::Greeting;

    while state != GenericManagement::Exit {
        match state {
            GenericManagement::Greeting => {
                writeln!(writer, "Generic management")?;
            }
            GenericManagement::Report => {
                writeln!(writer, "{}", device.report_to_string())?;
            }
            _ => {
                wrong_command(writer)?;
            }
        }

        writeln!(writer, "{}", ["1 - Report", "2 - Back"].join("\n"))?;

        state = match get_input() {
            Result::Ok(input) => match input.as_ref() {
                "1" => GenericManagement::Report,
                "2" => GenericManagement::Exit,
                _ => GenericManagement::WrongCommand,
            },
            _ => GenericManagement::WrongCommand,
        };
    }

    Result::Ok(())
}

pub fn device_management(writer: &mut dyn io::Write, device: &Device) -> io::Result<()> {
    match device {
        Device::Outlet(outlet) => outlet_management(writer, outlet),
        Device::Thermometer(thermometer) => thermometer_management(writer, thermometer),
        Device::Generic(generic) => generic_management(writer, generic.as_ref()),
    }
}
