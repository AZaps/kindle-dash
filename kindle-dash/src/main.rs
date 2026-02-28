use std::process::Command;
use image::{GrayImage, Luma};

fn main() {
    env_logger::init();
    log::info!("Starting Kindle Dash...");

    // start with a shell and then expand to using Powershell if on Windows
    let command = "fbink --clear -x 10 -y 10 \"Hello World!\"";
    let output = Command::new("sh")
    .arg("-c")
    .arg(format!("ssh -i ~/.ssh/id_rsa root@192.168.50.203 '{}'", command))
    .output()
    .expect("failed to execute process");

    let connection = output.stdout;
    let connection = String::from_utf8_lossy(&connection);
    log::info!("Connection output: {:?}", connection);

    // Sleep for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Render the dashboard and save it as a PNG file
    let dashboard = render_dashboard();
    dashboard.save("dashboard.png").expect("Failed to save dashboard image");

    // Send the dashboard image to the Kindle using fbink
    let send_command = "fbink -g /mnt/us/dashboard/dashboard.png";
    let send_output = Command::new("sh")
    .arg("-c")
    .arg(format!("ssh -i ~/.ssh/id_rsa root@192.168.50.203 '{}'", send_command))
    .output()
    .expect("failed to execute process");

    let send_connection = send_output.stdout;
    let send_connection = String::from_utf8_lossy(&send_connection);
    log::info!("Send connection output: {:?}", send_connection);

}

fn render_dashboard() -> image::GrayImage {
    let imgx = 600;
    let imgy = 800;
    let mut imgbuf = GrayImage::new(imgx, imgy);
    
    // Draw a gray background
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = Luma([0u8]);
    }
    imgbuf
}
