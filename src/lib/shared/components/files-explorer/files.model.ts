import type { Miliseconds } from '$lib/shared/types';
import dayjs from 'dayjs';

export interface FSDirEntry {
    name: string;
    path: string;
    is_dir: boolean;
}

export type BaseProjectFile = {
    id: string;
    projectId: string;
    name: string;
    path: string;
    createdAt: dayjs.Dayjs;
    updatedAt: dayjs.Dayjs;
};

export type VideoFrame = {
    id: string;
    name: string;
    path: string;
}

export type VideoProjectFile = BaseProjectFile & {
    frames: VideoFrame[];
    duration: Miliseconds;
}

export type ProjectFile = VideoProjectFile;

export type RawProjectFile = {
    project_file: BaseProjectFile;
    duration: number;
    frames: VideoFrame[];
}