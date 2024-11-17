abstract class SvelteState<T> {
    // private _logger: Logger = new Logger(isDev(), this.constructor.name, LogLevel.ERROR);
    private _state: T;

    init() {}

    protected get state(): $state.Snapshot<T> {
        return $state.snapshot(this._state);
    }

    protected constructor(initialState: T) {
        this._state = $state<T>(initialState);
    }

    protected select<K>(mapFn: (state: T) => K): K {
        return mapFn(this._state);
    }

    protected setState(newState: Partial<T>): void {
        this._state = Object.assign({}, this._state, newState);
    }
}
