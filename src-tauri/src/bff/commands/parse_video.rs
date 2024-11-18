use ffmpeg_next::format::input;
use ffmpeg_next::frame::Video;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::format::pixel::Pixel;
use image::{ImageBuffer, ImageEncoder, Rgb};
use std::path::Path;
use tauri::command;

#[command]
pub fn parse_video(src: String) -> Result<Vec<Vec<u8>>, String> {
    // Initialize ffmpeg
    ffmpeg_next::init().map_err(|e| e.to_string())?;

    // Open the video file
    let mut ictx = input(&Path::new(&src)).map_err(|e| e.to_string())?;

    // Find the video stream
    let input = ictx
        .streams()
        .best(ffmpeg_next::media::Type::Video)
        .ok_or_else(|| "Could not find video stream".to_string())?;

    // Extract the necessary information from the video stream
    let stream_index = input.index();
    let parameters = input.parameters();

    // Create a decoder for the video stream
    let mut decoder = ffmpeg_next::codec::context::Context::from_parameters(parameters)
        .map_err(|e| e.to_string())?
        .decoder()
        .video()
        .map_err(|e| e.to_string())?;

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

    // Vector to store the frames as PNG images
    let mut frames = Vec::new();

    // Create a mutable reference to ictx.packets()
    let mut packets = ictx.packets();

    // Iterate over the packets in the video stream
    while let Some((stream, packet)) = packets.next() {
        if stream.index() == stream_index {
            // Decode the packet into a frame
            decoder.send_packet(&packet).map_err(|e| e.to_string())?;
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                // Convert the frame to RGB format
                let mut rgb_frame = Video::empty();
                scaler
                    .run(&decoded, &mut rgb_frame)
                    .map_err(|e| e.to_string())?;

                // Convert the frame to an image buffer
                let buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(
                    rgb_frame.width(),
                    rgb_frame.height(),
                    rgb_frame.data(0).to_vec(),
                )
                .ok_or_else(|| "Failed to create image buffer".to_string())?;

                // Encode the image buffer as PNG
                let mut png_data = Vec::new();
                image::codecs::png::PngEncoder::new(&mut png_data)
                    .write_image(
                        &buffer,
                        buffer.width(),
                        buffer.height(),
                        image::ExtendedColorType::Rgb8,
                    )
                    .map_err(|e| e.to_string())?;

                frames.push(png_data);
            }
        }
    }

    // Flush the decoder
    decoder.send_eof().map_err(|e| e.to_string())?;
    let mut decoded = Video::empty();
    while decoder.receive_frame(&mut decoded).is_ok() {
        let mut rgb_frame = Video::empty();
        scaler
            .run(&decoded, &mut rgb_frame)
            .map_err(|e| e.to_string())?;

        let buffer: ImageBuffer<Rgb<u8>, _> = ImageBuffer::from_raw(
            rgb_frame.width(),
            rgb_frame.height(),
            rgb_frame.data(0).to_vec(),
        )
        .ok_or_else(|| "Failed to create image buffer".to_string())?;

        let mut png_data = Vec::new();
        image::codecs::png::PngEncoder::new(&mut png_data)
            .write_image(
                &buffer,
                buffer.width(),
                buffer.height(),
                image::ExtendedColorType::Rgb8,
            )
            .map_err(|e| e.to_string())?;

        frames.push(png_data);
    }

    Ok(frames)
}
