import { ComponentChildren, Context, createContext, FunctionalComponent } from "preact";
import { EmulatorManager } from "../emulator/emulator-manager";
import { h } from "preact";
import { useEffect, useState } from "preact/hooks";
import { useInitializeEmulator } from "../hooks/useInitializeEmulator";

// Any global options can live here
interface EmulatorContext {
    emulator?: EmulatorManager;
    /** has the emulator been initialized */
    loading: boolean;
}

interface Props {
    children?: ComponentChildren;
}

const Emulator = createContext<EmulatorContext>({
    loading: false
});


/** Retrieve emulator context object */
export const getEmulatorContext = (): Context<EmulatorContext> => {
    return Emulator;
}


/** Provider which handles linking components to emulator context */
export const EmulatorProvider: FunctionalComponent<Props> = ({ children }: Props) => {

    const { emulator, loading, error } = useInitializeEmulator();

    // TODO: utilize errorboundary here
    if (error) {
        console.log(error);
    }

    return (
        <Emulator.Provider value={{ emulator, loading }}>
            {children}
        </Emulator.Provider>
    )

}

