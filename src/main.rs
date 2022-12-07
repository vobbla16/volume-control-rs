use clap::{Parser, Subcommand};
use notify_rust::{Notification, Hint};
use pulsectl::controllers::{SinkController, DeviceControl};

const VOLUME_NOTIFICATION_SUMMARY: &str = "Volume";
const VOLUME_NOTIFICATION_ID: u32 = 2022;
const VOLUME_NOTIFICATION_APPNAME: &str = "volume-control-rs";
const VOLUME_NOTIFICATION_TIMEOUT: i32 = 1300;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Increase volume by given percent")]
    Increase { percent: f64 },

    #[command(about = "Decrease volume by given percent")]
    Decrease { percent: f64 },
}

fn show_volume_notification(volume: u32) {
    _ =  Notification::new()
       .summary(VOLUME_NOTIFICATION_SUMMARY)
       .body(&format!("{}%", volume.to_owned()))
       .hint(Hint::CustomInt("value".to_string(), volume.try_into().unwrap()))
       .appname(VOLUME_NOTIFICATION_APPNAME)
       .timeout(VOLUME_NOTIFICATION_TIMEOUT)
       .id(VOLUME_NOTIFICATION_ID)
       .show();
}


fn main() {
    let cli = Cli::parse();
    let mut handler = SinkController::create().unwrap();
    let default_device_index = handler.get_default_device().unwrap().index;

    match &cli.command {
        Commands::Increase { percent } => {
            handler.increase_device_volume_by_percent(default_device_index, percent.to_owned()/100.0);

            let volume = handler.get_default_device().unwrap().volume.avg().0/655;
            show_volume_notification(volume);
       } 
       Commands::Decrease { percent } => {
            handler.decrease_device_volume_by_percent(default_device_index, percent.to_owned()/100.0);

            let volume = handler.get_default_device().unwrap().volume.avg().0/655;
            show_volume_notification(volume);
            println!("{}", volume);
       }
    }
}