use crate::be::file_system::file_info;
use crate::models::ProjectFile;
use crate::schema::project_files;
// use crate::schema::project_files::dsl::*;
use base64::encode;
use diesel::prelude::*;
use image::codecs::png::PngEncoder;
use image::{ColorType, ImageBuffer, ImageEncoder, RgbImage};
use serde::Serialize;
use std::io::Cursor;
use uuid::Uuid;

extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::env;
use std::fs::File;
use std::io::prelude::*;

pub struct ProjectFilesRepository;

#[derive(Serialize, Debug)]
pub struct Frame {
    pub id: String,
    pub name: String,
    pub data: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectFileQuery {
    pub project_file: ProjectFile,
    pub frames: Vec<Frame>,
}

impl ProjectFilesRepository {
    pub fn new() -> Self {
        ProjectFilesRepository
    }

    // pub fn load_account_by_id(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id: &str,
    // ) -> Result<Account, diesel::result::Error> {
    //     accounts.find(account_id).first::<Account>(conn)
    // }

    pub fn load_project_files(
        &self,
        conn: &mut SqliteConnection,
        searched_project_id: &str,
    ) -> Result<Vec<ProjectFileQuery>, diesel::result::Error> {
        let project_files = project_files::table
            .filter(project_files::project_id.eq(searched_project_id))
            .order(project_files::columns::created_at.desc())
            .load::<ProjectFile>(conn);

        let mut project_file_queries = Vec::new();
        project_files?.iter().for_each(|project_file| {
            match ProjectFilesRepository::extract_frames(&project_file.path) {
                Ok(frames) => project_file_queries.push(ProjectFileQuery {
                    project_file: project_file.clone(),
                    frames,
                }),
                Err(e) => eprintln!("Failed to extract frames: {}", e),
            }

            Ok::<(), Box<dyn std::error::Error>>(());
        });

        Ok(project_file_queries)
    }

    pub fn insert_project_file(
        &self,
        conn: &mut SqliteConnection,
        project_id: String,
        file_path: String,
    ) -> Result<ProjectFile, diesel::result::Error> {
        let file_info = file_info::resolve_for(&file_path).expect("Failed to resolve file info");
        let new_project_file = ProjectFile {
            id: Uuid::new_v4().to_string(),
            project_id: project_id,
            name: file_info.name,
            path: file_info.path,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(project_files::table)
            .values(&new_project_file)
            .execute(conn)?;

        Ok(new_project_file)
    }

    pub fn extract_frames(video_path: &str) -> Result<Vec<Frame>, Box<dyn std::error::Error>> {
        print!("decoder 0");
        ffmpeg::init()?;

        let mut frames = Vec::new();
        if let Ok(mut ictx) = input(&video_path) {
            let input = ictx
                .streams()
                .best(Type::Video)
                .ok_or(ffmpeg::Error::StreamNotFound)?;
            let video_stream_index = input.index();

            print!("input parameters: {:?}", input);
            let context_decoder =
                ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
            let mut decoder = context_decoder.decoder().video()?;

            let mut scaler = Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                Pixel::RGB24,
                decoder.width(),
                decoder.height(),
                Flags::BILINEAR,
            )?;

            let mut frame_index = 0;

            let mut receive_and_process_decoded_frames =
                |decoder: &mut ffmpeg::decoder::Video| -> Result<(), ffmpeg::Error> {
                    let mut decoded = Video::empty();
                    while decoder.receive_frame(&mut decoded).is_ok() {
                        print!("received frame {} \n", frame_index);
                        let mut rgb_frame = Video::empty();
                        scaler.run(&decoded, &mut rgb_frame)?;

                        let img_base64 = encode(&rgb_frame.data(0));

                        frames.push(Frame {
                            id: frame_index.to_string(),
                            name: format!("frame_{}", frame_index),
                            data: img_base64,
                        });

                        frame_index += 1;
                    }
                    Ok(())
                };

            for (stream, packet) in ictx.packets() {
                if stream.index() == video_stream_index {
                    decoder.send_packet(&packet)?;
                    receive_and_process_decoded_frames(&mut decoder)?;
                }
            }
            decoder.send_eof()?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }

        Ok(frames)
    }

    // pub fn update_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_update: AccountUpdate, // Assuming AccountUpdate is the struct for updating account data
    // ) -> Result<Account, diesel::result::Error> {
    //     use crate::schema::accounts::dsl::*;

    //     // Clone the id before unwrapping it
    //     let account_id = account_update.id.clone().unwrap();

    //     diesel::update(accounts.find(account_id))
    //         .set(&account_update)
    //         .execute(conn)?;

    //     accounts
    //         .filter(id.eq(account_update.id.unwrap()))
    //         .first(conn)
    // }

    // pub fn delete_account(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_id: String, // ID of the account to delete
    // ) -> Result<usize, diesel::result::Error> {
    //     use crate::schema::accounts::dsl::*;

    //     diesel::delete(accounts.filter(id.eq(account_id))).execute(conn)
    // }

    // pub fn load_accounts_by_ids(
    //     &self,
    //     conn: &mut SqliteConnection,
    //     account_ids: &[String],
    // ) -> Result<Vec<Account>, diesel::result::Error> {
    //     accounts
    //         .filter(id.eq_any(account_ids))
    //         .filter(is_active.eq(true))
    //         .order(created_at.desc())
    //         .load::<Account>(conn)
    // }
}
