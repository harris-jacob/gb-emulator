import { FunctionalComponent, h } from 'preact';
import styled from 'styled-components';
import InstructionList from '../../components/debugger/instruction-list';
import NextInstruction from '../../components/debugger/next-instruction';
import RegisterDisplay from '../../components/debugger/register-display';
import { EmulatorProvider } from '../../components/EmulatorContext';
import { useEmulator } from '../../hooks/useEmulator';

const Container: FunctionalComponent = styled.div`
    display: flex;
    > * {
            &:last-child {
            flex-grow: 1; 
        }
    }
`

const Page: FunctionalComponent = () => {
    return (
        <EmulatorProvider>
            <RootComponent />
        </EmulatorProvider>
    );
};

const RootComponent = () => {
    const { step, registers, loading, instructionList, nextInstruction } = useEmulator();
    return (
        <Container>
            {loading && <div>Loading...</div>}
            <div>
                {registers && <RegisterDisplay register={registers} />}
                {!loading && nextInstruction && <NextInstruction step={step} nextInstruction={nextInstruction} />}
            </div>
            {!loading && instructionList && registers && <InstructionList pc={registers.pc()} instructions={instructionList} />}
        </Container>
    )
}

export default Page;
