import React from "react";
import styled from "styled-components";
import InstructionList from "../../components/debugger/instruction-list";
import NextInstruction from "../../components/debugger/next-instruction";
import RegisterDisplay from "../../components/debugger/register-display";
import { EmulatorProvider } from "../../components/EmulatorContext";
import { useDebugger } from "../../hooks/useDebugger";

const Container = styled.div`
  display: flex;
  > * {
    &:last-child {
      flex-grow: 1;
    }
  }
`;

const Page: React.FC = () => {
  return (
    <EmulatorProvider>
      <RootComponent />
    </EmulatorProvider>
  );
};

const RootComponent = () => {
  const {
    step,
    registers,
    loading,
    instructionList,
    nextInstruction,
    toggleRun,
    isRunning,
  } = useDebugger();
  return (
    <Container>
      {loading && <div>Loading...</div>}
      <div>
        {registers && <RegisterDisplay register={registers} />}
        {!loading && nextInstruction && (
          <NextInstruction
            toggleRun={toggleRun}
            isRunning={isRunning}
            step={step}
            nextInstruction={nextInstruction}
          />
        )}
      </div>
      {!loading && instructionList && registers && (
        <InstructionList pc={registers.pc()} instructions={instructionList} />
      )}
    </Container>
  );
};

export default Page;
