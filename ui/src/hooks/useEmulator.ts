import { EmulatorManager } from "../emulator/emulator-manager"
import { useEffect, useState } from "preact/hooks";

interface UseEmulatorReturn {
    /** is the emulator still initializing? */ 
    loading: boolean;
    /** Has the emulator errored */
    error?: Error
    /** Emulator Instance */ 
    emulator?: EmulatorManager;
}

export const useEmulator = (): UseEmulatorReturn => {
    const [emulator, setEmulator] = useState<EmulatorManager>()
    const [ loading, setLoading ] = useState(false);
    const [ error, setError ] = useState<Error>();

    useEffect(() => {
        const emulatorInstance = new EmulatorManager();
        const init = async () => {
            setLoading(true);
            await emulatorInstance.init();
            setLoading(false);
            setEmulator(emulatorInstance);
        }

        try {
            init();
        } catch (e) {
            setError(e as Error)
        }

    }, [])


    return {loading, error, emulator};
}