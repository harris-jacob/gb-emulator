import { getBit } from "../utils/getBit";
import { EmulatorAPI } from "./types";

/** Provides view access to emulator register values */
export class RegisterView {
    /** A number which points to the array index at which the register object lives in WASM memory */
    address: number; 
    /** 16 bit view into WASM heap segment */    
    heap16: number[];
    /** 8 bit view into WASM heap segment */    
    heap8: number[]; 
    
    /**
     * @param address array index of WASM memory that register begins
     */
    constructor(emulator: EmulatorAPI) {
        this.address = emulator._get_reg();
        this.heap8 = emulator.HEAPU8;
        this.heap16 = emulator.HEAPU16;
    }

    public af(): number {
        return this.heap16[this.address >> 1];
    }
    public bc(): number {
        return this.heap16[(this.address >> 1) +1];
    } 
    public de(): number {
        return this.heap16[(this.address >> 1) + 2];
    } 
    public hl(): number {
        return this.heap16[(this.address >> 1) + 3];
    } 
    public sp(): number {
        return this.heap16[(this.address >> 1) + 4];
    } 
    public pc(): number {
        return this.heap16[(this.address >> 1) + 5];
    }
    public a(): number {
        return this.heap8[this.address];
    }
    public b(): number {
        return this.heap8[this.address + 2];
    }
    public c(): number {
        return this.heap8[this.address + 3];
    }
    public d(): number {
        return this.heap8[this.address + 4];
    }
    public e(): number {
        return this.heap8[this.address + 5];
    }
    public h(): number {
        return this.heap8[this.address + 6];
    }
    public l(): number {
        return this.heap8[this.address + 7];
    }

    public isCarrySet(): boolean {
        const flags = this.f() 
        return getBit(flags, 4) === 1;
    }
    
    public isHalfCarrySet(): boolean {
        const flags = this.f() 
        return getBit(flags, 5) === 1;
    }
    
    public isSubtractSet(): boolean {
        const flags = this.f() 
        return getBit(flags, 6) === 1;
    }
    
    public isZeroSet(): boolean {
        const flags = this.f() 
        return getBit(flags, 7) === 1;
    }

    private f(): number {
        return this.heap8[this.address + 1];
    }
}