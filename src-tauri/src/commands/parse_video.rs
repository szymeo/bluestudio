use ffmpeg_next::format::input;
use ffmpeg_next::frame::Video;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::format::pixel::Pixel;
use std::path::Path;

pub fn parse_video(src: &str) -> Result<Vec<Video>, String> {
    // Initialize ffmpeg
    ffmpeg_next::init().map_err(|e| e.to_string())?;

    // Open the video file
    let mut ictx = input(&Path::new(src)).map_err(|e| e.to_string())?;

    // Find the video stream
    let input = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .ok_or_else(|| "Could not find video stream".to_string())?;

    // Create a decoder for the video stream
    let mut decoder = input.codec().decoder().video().map_err(|e| e.to_string())?;

    // Create a scaler to convert frames to RGB format
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )
    .map_err(|e| e.to_string())?;

    // Vector to store the frames
    let mut frames = Vec::new();

    // Iterate over the packets in the video stream
    for (stream, packet) in ictx.packets() {
        if stream.index() == input.index() {
            // Decode the packet into a frame
            decoder.send_packet(&packet).map_err(|e| e.to_string())?;
            while let Ok(mut frame) = decoder.receive_frame() {
                // Convert the frame to RGB format
                let mut rgb_frame = Video::empty();
                scaler
                    .run(&frame, &mut rgb_frame)
                    .map_err(|e| e.to_string())?;
                frames.push(rgb_frame);
            }
        }
    }

    // Flush the decoder
    decoder.send_eof().map_err(|e| e.to_string())?;
    while let Ok(mut frame) = decoder.receive_frame() {
        let mut rgb_frame = Video::empty();
        scaler
            .run(&frame, &mut rgb_frame)
            .map_err(|e| e.to_string())?;
        frames.push(rgb_frame);
    }

    Ok(frames)
}
