export type Branded<T, U> = T & { __brand: U };

export type Miliseconds = Branded<number, 'Miliseconds'>;