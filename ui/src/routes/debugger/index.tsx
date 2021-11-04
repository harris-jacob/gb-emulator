import { FunctionalComponent, h } from 'preact';
import RegisterDisplay from '../../components/debugger/register-display';
import { EmulatorProvider } from '../../components/EmulatorContext';
import Button from '../../components/generic/button';
import { useEmulator } from '../../hooks/useEmulator';

const Page: FunctionalComponent = () => {
    return (
        <EmulatorProvider>
            <RootComponent />
        </EmulatorProvider>
    );
};


const RootComponent = () => {
    const { step, registers, loading, rom } = useEmulator();

    console.log(rom);
    return (
        <div>
            {loading && <div>Loading...</div>}
            {registers && <RegisterDisplay register={registers} />}
            {!loading && <Button onClick={step}>STEP</Button>}
        </div>
    )
}

export default Page;
