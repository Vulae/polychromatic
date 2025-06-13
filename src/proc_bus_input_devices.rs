//! Parses output from /proc/bus/input/devices

#![allow(unused)]

use std::collections::HashMap;

use crate::PolychromaticError;

#[derive(Debug)]
pub struct BusInputDeviceId {
    pub bus_type: u16,
    pub vendor: u16,
    pub product: u16,
    pub version: u16,
}

#[derive(Debug)]
pub struct BusInputUnparsedField {
    pub r#type: char,
    pub value: String,
}

#[derive(Debug)]
pub struct BusInputDevice {
    pub id: BusInputDeviceId,
    pub name: String,
    pub unparsed_fields: Box<[BusInputUnparsedField]>,
}

pub fn query_input_devices() -> Result<Box<[BusInputDevice]>, PolychromaticError> {
    let str = std::fs::read_to_string("/proc/bus/input/devices")?;
    let devices_str = str.split("\n\n");

    devices_str
        .into_iter()
        .filter(|device_str| !device_str.is_empty())
        .map(|device_str| {
            let mut raw = Vec::new();
            let mut id = None;
            let mut name = None;

            for device_prop in device_str.lines() {
                let [r#type, ':', ' '] = device_prop
                    .get(0..3)
                    .ok_or(PolychromaticError::CannotParseDevice(
                        "Failed to parse device field",
                    ))?
                    .chars()
                    .collect::<Vec<_>>()[..]
                else {
                    return Err(PolychromaticError::CannotParseDevice(
                        "Failed to parse device field",
                    ));
                };
                let value = device_prop
                    .get(3..)
                    .ok_or(PolychromaticError::CannotParseDevice(
                        "Failed to parse device field",
                    ))?
                    .to_owned();

                match r#type {
                    'I' => {
                        let ids: HashMap<String, u16> = value
                            .split(' ')
                            .map(|id_str| {
                                let [id_type_str, id_value_str] =
                                    id_str.split('=').collect::<Vec<_>>()[..]
                                else {
                                    return Err(PolychromaticError::CannotParseDevice(
                                        "Device ID field malformed",
                                    ));
                                };
                                Ok((
                                    id_type_str.to_owned(),
                                    u16::from_str_radix(id_value_str, 16).map_err(|_| {
                                        PolychromaticError::CannotParseDevice(
                                            "ID field could not parse ID number from subfield",
                                        )
                                    })?,
                                ))
                            })
                            .collect::<Result<_, _>>()?;
                        id = Some(BusInputDeviceId {
                            bus_type: *ids
                                .iter()
                                .find(|(k, _)| k.as_str() == "Bus")
                                .ok_or(PolychromaticError::CannotParseDevice(
                                    "ID field could not find bustype subfield",
                                ))?
                                .1,
                            vendor: *ids
                                .iter()
                                .find(|(k, _)| k.as_str() == "Vendor")
                                .ok_or(PolychromaticError::CannotParseDevice(
                                    "ID field could not find vendor subfield",
                                ))?
                                .1,
                            product: *ids
                                .iter()
                                .find(|(k, _)| k.as_str() == "Product")
                                .ok_or(PolychromaticError::CannotParseDevice(
                                    "ID field could not find product subfield",
                                ))?
                                .1,
                            version: *ids
                                .iter()
                                .find(|(k, _)| k.as_str() == "Version")
                                .ok_or(PolychromaticError::CannotParseDevice(
                                    "ID field could not find version subfield",
                                ))?
                                .1,
                        });
                    }
                    'N' => {
                        let Some(value) = value.strip_prefix("Name=") else {
                            return Err(PolychromaticError::CannotParseDevice(
                                "Name field invalid prefix",
                            ));
                        };
                        if value.starts_with('"') && value.ends_with('"') {
                            name = Some(value.get(1..(value.len() - 1)).unwrap().to_owned());
                        } else {
                            name = Some(value.to_owned());
                        }
                    }
                    _ => {
                        raw.push(BusInputUnparsedField {
                            r#type,
                            value: value.to_owned(),
                        });
                    }
                }
            }

            Ok(BusInputDevice {
                id: id.ok_or(PolychromaticError::CannotParseDevice(
                    "Device ID field is required",
                ))?,
                name: name.ok_or(PolychromaticError::CannotParseDevice(
                    "Device name field is required",
                ))?,
                unparsed_fields: raw.into_boxed_slice(),
            })
        })
        .collect::<Result<Box<[_]>, PolychromaticError>>()
}
