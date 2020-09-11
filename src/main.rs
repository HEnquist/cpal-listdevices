extern crate anyhow;
extern crate cpal;

use cpal::traits::{DeviceTrait, HostTrait};

//fn print_list(items: &[str]) {
//    for item in items.iter() {
//        println!("    {}",item);
//    }
//}

fn display_sample_format(sformat: &cpal::SampleFormat) -> &'static str {
    match sformat {
        cpal::SampleFormat::F32 => "FLOAT32LE",
        cpal::SampleFormat::I16 => "S16LE",
        cpal::SampleFormat::U16 => "U16LE",
    }
}

fn print_supported_conf(conf: &cpal::SupportedStreamConfig) {
    let channels = conf.channels();
    let sample_rate = conf.sample_rate();
    let sformat = display_sample_format(&conf.sample_format());
    println!("      channels: {}, samplerate: {}, format: {}", channels, sample_rate.0, sformat);
}

fn print_conf_range(conf: &cpal::SupportedStreamConfigRange) {
    let channels = conf.channels();
    let sample_rate_min = conf.min_sample_rate();
    let sample_rate_max = conf.max_sample_rate();
    let sformat = display_sample_format(&conf.sample_format());
    println!("      channels: {}, samplerate min: {} max: {}, format: {}", channels, sample_rate_min.0, sample_rate_max.0, sformat);
}

fn main() -> Result<(), anyhow::Error> {
    let available_hosts = cpal::available_hosts();
    println!("Available hosts:\n  {:?}", available_hosts);

    for host_id in available_hosts {
        println!("{}", host_id.name());
        let host = cpal::host_from_id(host_id)?;
        if let Some(default_in) = host.default_input_device() {
            println!("  Default Input Device:\n    {:?}", default_in.name());
        } else {
            println!("  Failed getting Default Input Device");
        }
        if let Some(default_out) = host.default_output_device() {
            println!("  Default Output Device:\n    {:?}", default_out.name());
        } else {
            println!("  Failed getting Default Output Device");
        }

        let devices = host.devices()?;
        println!("  Devices: ");
        for (_device_index, device) in devices.enumerate() {
            println!("\n\n  Device: \"{}\"", device.name()?);
            println!("  ============================================================");
            println!("\n    Capture\n    ------------------------------------------------------------");
            if let Ok(conf) = device.default_input_config() {
                println!("    Default input stream config:");
                print_supported_conf(&conf);
            }
            let mut input_configs = match device.supported_input_configs() {
                Ok(f) => f.peekable(),
                Err(e) => {
                    println!("    Error: {:?}", e);
                    continue;
                }
            };
            if input_configs.peek().is_some() {
                println!("    All supported input stream configs:");
                for (_config_index, config) in input_configs.enumerate() {
                    print_conf_range(&config);
                }
            }
            println!("\n    Playback\n    ------------------------------------------------------------");
            if let Ok(conf) = device.default_output_config() {
                println!("    Default output stream config:");
                print_supported_conf(&conf);
            }
            let mut output_configs = match device.supported_output_configs() {
                Ok(f) => f.peekable(),
                Err(e) => {
                    println!("    Error: {:?}", e);
                    continue;
                }
            };
            if output_configs.peek().is_some() {
                println!("    All supported output stream configs:");
                for (_config_index, config) in output_configs.enumerate() {
                    print_conf_range(&config);
                }
            }
        }
    }

    Ok(())
}
