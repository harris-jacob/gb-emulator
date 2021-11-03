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
}

