use crate::be::file_system::file_info;
use crate::models::ProjectFile;
use crate::schema::project_files;
use diesel::prelude::*;
use image::{ColorType, ImageBuffer, ImageEncoder, RgbImage};
use serde::Serialize;
use std::path::Path;
use uuid::Uuid;

extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use std::fs::File;
use std::io::prelude::*;

pub struct ProjectFilesRepository;

#[derive(Serialize, Debug)]
pub struct Frame {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Debug)]
pub struct ProjectFileQuery {
    pub project_file: ProjectFile,
    pub frames: Vec<Frame>,
    pub duration: f64,
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
            .load::<ProjectFile>(conn)
            .expect("Failed to load project files");

        println!("Amount of project files: {}", project_files.len());

        let mut project_file_queries = Vec::new();
        for project_file in &project_files {
            match ProjectFilesRepository::extract_frames(&searched_project_id, &project_file.path) {
                Ok(frames) => project_file_queries.push(ProjectFileQuery {
                    project_file: project_file.clone(),
                    frames,
                    duration: ProjectFilesRepository::get_duration(&project_file.path)
                        .expect("Failed to get duration"),
                }),
                Err(e) => eprintln!("Failed to extract frames: {}", e),
            }
        }

        print!("project_file_queries: {:?}", &project_file_queries.len());

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

    pub fn get_duration(video_path: &str) -> Result<f64, Box<dyn std::error::Error>> {
        ffmpeg::init()?;

        let mut duration = 0.0;
        if let Ok(mut ictx) = input(&video_path) {
            let input = ictx
                .streams()
                .best(Type::Video)
                .ok_or(ffmpeg::Error::StreamNotFound)
                .expect("Failed to find video stream");

            let time_base = input.time_base();
            duration = input.duration() as f64
                * (time_base.numerator() as f64 / time_base.denominator() as f64)
                * 1000.0;
        }

        Ok(duration)
    }

    pub fn extract_frames(
        searched_project_id: &str,
        video_path: &str,
    ) -> Result<Vec<Frame>, Box<dyn std::error::Error>> {
        let video_name = video_path.split('/').last().unwrap();
        let frame_dir = format!(
            "tmp_bstd_data/project_{}/{}/",
            searched_project_id, video_name
        );

        // Check if frames already exist
        if Path::new(&frame_dir).exists() {
            let mut frames = Vec::new();
            for entry in std::fs::read_dir(&frame_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    let frame_index = path
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace("frame", "")
                        .parse::<usize>()
                        .unwrap();
                    frames.push(Frame {
                        id: frame_index.to_string(),
                        name: format!("frame_{}", frame_index),
                        path: path.to_string_lossy().to_string(),
                    });
                }
            }
            frames.sort_by(|a, b| {
                a.id.parse::<usize>()
                    .unwrap()
                    .cmp(&b.id.parse::<usize>().unwrap())
            });
            return Ok(frames);
        }

        // If frames do not exist, run the decoder
        ffmpeg::init()?;

        let mut frames = Vec::new();
        if let Ok(mut ictx) = input(&video_path) {
            let input = ictx
                .streams()
                .best(Type::Video)
                .ok_or(ffmpeg::Error::StreamNotFound)
                .expect("Failed to find video stream");
            let video_stream_index = input.index();

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
                        let mut rgb_frame = Video::empty();
                        scaler
                            .run(&decoded, &mut rgb_frame)
                            .expect("Failed to run scaler");

                        let path = format!("{}frame{}.ppm", frame_dir, frame_index);
                        save_file(&rgb_frame, &path).unwrap();

                        frames.push(Frame {
                            id: frame_index.to_string(),
                            name: format!("frame_{}", frame_index),
                            path,
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
}

fn save_file(frame: &Video, path: &str) -> std::result::Result<(), std::io::Error> {
    let path = Path::new(path);

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path).expect("Failed to create file");
    file.write_all(format!("P6\n{} {}\n255\n", frame.width(), frame.height()).as_bytes())?;
    file.write_all(frame.data(0))?;
    Ok(())
}
