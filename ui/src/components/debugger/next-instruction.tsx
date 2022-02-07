import React from "react";
import { Instruction } from "../../emulator/types";
import Button from "../generic/button";
import Card from "../generic/card";
import H6 from "../generic/h6";

interface Props {
  nextInstruction: Instruction;
  step: () => void;
  toggleRun: () => void;
  isRunning: boolean;
}

const instrucitonDisplay = (instruction: Instruction) => (
  <>
    {`opcode: ${instruction.opcode.toString(16)}`} <br />
    {`operation: ${instruction.name}`} <br />
    {`operand: ${instruction.operand}`} <br />
  </>
);

const NextInstruction: React.FC<Props> = (props: Props) => {
  const { nextInstruction, step, toggleRun, isRunning } = props;

  return (
    <Card>
      <H6>Next Instruction</H6>
      {instrucitonDisplay(nextInstruction)}
      <Button onClick={step}>EXECUTE</Button>
      <Button onClick={toggleRun}>{isRunning ? "RUN" : "PAUSE"} </Button>
    </Card>
  );
};

export default NextInstruction;
