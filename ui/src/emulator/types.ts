export interface EmulatorAPI {
    _get_reg(): number;
    _init(): void;
    _step(): void;
    _get_rom(): number;
    HEAPU16: number[];
    HEAPU8: number[];
}


export type EmulatorFactory = () => Promise<EmulatorAPI>;