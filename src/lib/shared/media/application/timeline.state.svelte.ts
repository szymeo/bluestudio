import type { TimelineTrack } from "../domain/timeline-track";

class TimelineState extends SvelteState<{
    currentTime: number;
    tracks: Array<TimelineTrack>;
}> {
    constructor() {
        super({
            currentTime: 0,
            tracks: []
        });
    }

    addTrack(track: TimelineTrack) {
        this.setState({
            tracks: [...this.state.tracks, track]
        });
    }
}

export const timelineState = new TimelineState();