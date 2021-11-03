import { FunctionalComponent, h } from 'preact';
import RegisterDisplay from '../../components/debugger/register-display';
import Button from '../../components/generic/button';
import { useEmulator } from '../../hooks/useEmulator';

const Debugger: FunctionalComponent = () => {
    const { emulator, loading } = useEmulator();
    return (
        <div>
            {loading && <div>Loading...</div>}
            {emulator && <RegisterDisplay register={emulator.createRegisterView()} />}
        </div>

    );
};

export default Debugger;
