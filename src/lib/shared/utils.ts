export const pad = (num: number, size: number): string => num.toString().padStart(size, '0');

export function formatTimeWithFrames(totalMiliseconds: number, frames_count: number): string {
    const totalSeconds: number = totalMiliseconds / 1000;
    const hours: number = Math.floor(totalSeconds / 3600);
    const minutes: number = Math.floor((totalSeconds % 3600) / 60);
    const seconds: number = Math.floor(totalSeconds % 60);
    const fps = frames_count / totalSeconds;
    const frames: number = Math.floor((totalSeconds % 1) * fps);

    return `${pad(hours, 2)}:${pad(minutes, 2)}:${pad(seconds, 2)}:${pad(frames, 2)}`;
}