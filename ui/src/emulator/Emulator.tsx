import { useEffect } from 'preact/hooks';
import { FunctionalComponent, h } from 'preact';

export interface EmulatorFactory {
    _peek_reg(): object;
}

const Emulator: FunctionalComponent = () => {
    useEffect(() => {
        console.log("Emulator Loading...");
        const load = async () => {
            const factory: any = require('./emulator');

            factory().then(() => console.log("Hello"))
        }
        load();

    }, [])

    return (
        <div>Hello Emulator</div>
    )
}

export default Emulator;