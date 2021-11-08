export interface EmulatorAPI {
    _get_reg(): number;
    _init(): void;
    _step(): void;
    _get_rom(): number;
    _get_next_op_operand(): number;
    _get_next_opcode(): number;
    _get_next_op_name(): number;
    _get_op_name_by_pc(pc: number): number;
    _get_opcode_by_pc(pc: number): number;
    _get_operand_by_pc(pc: number): number;
    _get_opcode_operand_size(opcode: number): number;
    HEAPU16: number[];
    HEAPU8: number[];
    UTF8ToString(ptr: number): string;
}

export interface Instruction {
    opcode: number;
    name: string;
    operand: number;
}

export type EmulatorFactory = () => Promise<EmulatorAPI>;