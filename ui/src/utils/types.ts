/**
 * Alias for ID of T
 */
export type ID<T> = string;

/**  Table of Ts */
export interface Table<T> {
  [k: string]: T;
}


/** anything with an ID */
interface HasID {
  id: string;
}

/** Thing with an added ID */
export type WithID<T> = T & { id: string };