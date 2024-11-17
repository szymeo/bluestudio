type Frame = {
    id: string;
}

class FramesState extends SvelteState<{ frames: Record<string, Frame> }> {
    constructor() {
        super({ frames: {} });
    }

    addFrame(name: string) {
        this.setState({
            frames: {
                ...this.state.frames,
                [name]: { id: name }
            }
        });
    }

    removeFrame(name: string) {
        const frames = { ...this.state.frames };
        delete frames[name];
        this.setState({ frames });
    }
}

export const framesState = new FramesState();