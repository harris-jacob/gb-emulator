import { useEffect } from 'preact/hooks';
import { FunctionalComponent, h } from 'preact';

export interface EmulatorFactory {
    _peek_reg(): object;
}

const Emulator: FunctionalComponent = () => {
    // useEffect(() => {
    //     console.log("Emulator Loading...");
    //     const factory: () => Promise<EmulatorFactory> = require('./emulator');

    //     factory().then((emulator: EmulatorFactory) => {
    //         console.log("Module Loaded");
    //         console.log(emulator._peek_reg());
    //     }).catch(() => console.log("Caught"))
    // }, [])

    return (
    <div>Hello Emulator</div>
    )
}

export default Emulator;