import { assertDefined } from "../utils/assert";
import { RegisterView } from "./registers";
import { EmulatorAPI, EmulatorFactory } from "./types";

export class EmulatorManager {
    private emulator: EmulatorAPI | undefined;
    private registerView: RegisterView | undefined;

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

   /** Get the memory segment for the currently loaded rom bank */
   public getRomMemory(): readonly number[] {
        assertDefined(this.emulator);
        const start = this.emulator._get_rom();
        // Size of rom bank memory
        const end = start + 32768;

        return this.emulator.HEAPU8.slice(start, end);
    }
}

