export enum TimelineTrackType {
    VIDEO = 'video',
    AUDIO = 'audio'
}

type VideoTrack = {
    type: TimelineTrackType.VIDEO;
    start: number;
    end: number;
    frames: string[];
};

type AudioTrack = {
    type: TimelineTrackType.AUDIO;
    start: number;
    end: number;
    src: string;
};

export type TimelineTrack = VideoTrack | AudioTrack;