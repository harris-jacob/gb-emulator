import { useContext, useMemo, useReducer } from "preact/hooks";
import { getEmulatorContext } from "../components/EmulatorContext";
import { RegisterView } from "../emulator/registers";
import { Instruction } from "../emulator/types";
import { assertDefined } from "../utils/assert";

type RomMemory = readonly number[];

export interface UseEmulatorReturn {
    /** are we still loading the emulator */
    loading: boolean;
    /** process the next cpu instruction and update the emulator */
    step: () => void;
    /** view of the current registers */
    registers?: RegisterView;
    /** Details of the next opcode in the rom */
    nextInstruction?: Instruction
    /** List of opcodes */
    instructionList?: Instruction[]
}


/** use the emulator instance  */
export const useEmulator = (): UseEmulatorReturn => {
    const context = useContext(getEmulatorContext());
    const {loading, emulator} = context;

    const [ _, forceUpdate ] = useReducer<number, void>(x => x + 1, 0);

    const registers = useMemo<RegisterView | undefined>(() => {

        if(emulator) {
            return emulator.getRegisterView();
        }
    }, [loading, emulator]);

    const instructionList = useMemo<Instruction[] | undefined>(() => {
        if(emulator) {
            return emulator.createInstructionList()
        }
    }, [loading, emulator])


    const step = () => {
        assertDefined(context.emulator, "emulator instance is not defined: wait for load and make sure to use the EmulatorContext");
        context.emulator.step()
        // our update function must force render so we are showing the latest data
        // we can't do this during a render, so we queue until after
        setTimeout(() => forceUpdate(), 0);
    }


    // save new instruction
    let nextInstruction: Instruction | undefined;

    if(emulator) {
        nextInstruction = emulator.getNextInstruction();
    }

    return { loading: context.loading, registers, step, instructionList,  nextInstruction }
}