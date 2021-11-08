import { assertDefined } from "../utils/assert";
import { Table } from "../utils/types";
import { RegisterView } from "./registers";
import { EmulatorAPI, EmulatorFactory, Instruction } from "./types";

export class EmulatorManager {
    private emulator: EmulatorAPI | undefined;
    private registerView: RegisterView | undefined;
    private romMemory: readonly number[] | undefined;

    /** Load the emulator interface */
    public async init(): Promise<void> {
        const factory: EmulatorFactory = require("./emulator");
        this.emulator = await factory()
        this.emulator._init();
    }

   /** Execute the next opcode int the program */
   public step(): void {
    assertDefined(this.emulator);
    this.emulator._step();
   }

   /** Returns a register instance which can be used to view register values */
   public createRegisterView(): RegisterView {
       assertDefined(this.emulator)

       if(this.registerView === undefined) {
           this.registerView = new RegisterView(this.emulator);
       }
       return this.registerView;
   }

   public getNextInstruction(): Instruction {
       assertDefined(this.emulator);
       const namePtr = this.emulator._get_next_op_name();
       const name = this.emulator.UTF8ToString(namePtr);       

       const opcode = this.emulator._get_next_opcode();
       const operand = this.emulator._get_next_op_operand();

       return {name, opcode, operand }
    }

   /** Get the memory segment for the currently loaded rom bank */
   public getRomMemory(): readonly number[] {
        assertDefined(this.emulator);

        if(!this.romMemory) {
            const start = this.emulator._get_rom();
            // Size of rom bank memory
            const end = start + 32768;

            this.romMemory = this.emulator.HEAPU8.slice(start, end);
        }

        return this.romMemory;
    }

    /** 
     * Iterate over ROM memory and get the op list
     * This function should only run once per ROM load
     */
    public createInstructionList(): Table<Instruction> {
        assertDefined(this.emulator);
        const ops: Table<Instruction> = {}
        const memory = this.getRomMemory();
        const registers = this.createRegisterView();

        for(let pc=0x101; pc<= 0x7fff;) {
            // Build opcode object
            const opcode = this.emulator._get_opcode_by_pc(pc);
            const namePtr = this.emulator._get_op_name_by_pc(pc);
            const operand = this.emulator._get_operand_by_pc(pc);
            ops[pc] = {operand, name: this.emulator.UTF8ToString(namePtr), opcode}
            const operandSize = this.emulator._get_opcode_operand_size(opcode);
            
            switch(operandSize) {
                case 0:
                    pc++;
                    break;
                case 1:
                    pc+=1;
                    break;
                case 2:
                    pc+=2;
                    break;
                default:
                    throw new Error("Unknown operand size")
            }
        
        }

        return ops;
    }

    private getInstructionFromPC(pc: number): Instruction {
        assertDefined(this.emulator);
        const namePtr = this.emulator._get_op_name_by_pc(pc);
        const name = this.emulator.UTF8ToString(namePtr);

        const opcode = this.emulator._get_opcode_by_pc(pc);
        const operand = this.emulator._get_operand_by_pc(pc);

        return {name, opcode, operand}

    }
}

