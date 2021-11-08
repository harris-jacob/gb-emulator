import { FunctionalComponent, h } from 'preact';
import InstructionList from '../../components/debugger/instruction-list';
import NextInstruction from '../../components/debugger/next-instruction';
import RegisterDisplay from '../../components/debugger/register-display';
import { EmulatorProvider } from '../../components/EmulatorContext';
import { useEmulator } from '../../hooks/useEmulator';

const Page: FunctionalComponent = () => {
    return (
        <EmulatorProvider>
            <RootComponent />
        </EmulatorProvider>
    );
};


const RootComponent = () => {
    const { step, registers, loading, instructionList, nextInstruction } = useEmulator();
    console.log(instructionList);
    return (
        <div>
            {loading && <div>Loading...</div>}
            {registers && <RegisterDisplay register={registers} />}
            {!loading && nextInstruction && <NextInstruction step={step} nextInstruction={nextInstruction} />}
            {!loading && instructionList && <InstructionList instructions={instructionList} />}
        </div>
    )
}

export default Page;
