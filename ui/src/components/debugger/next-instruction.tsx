import { Fragment, FunctionalComponent, h } from "preact";
import { Instruction } from "../../emulator/types";
import Button from "../generic/button";
import Card from "../generic/card";
import H6 from "../generic/h6";

interface Props {
    nextInstruction: Instruction;
    step: () => void;
}

const instrucitonDisplay = (instruction: Instruction) => (
    <Fragment>
        {`opcode: ${instruction.opcode.toString(16)}`} <br />
        {`operation: ${instruction.name}`} <br />
        {`operand: ${instruction.operand}`} <br />
    </Fragment>
)

const NextInstruction: FunctionalComponent<Props> = (props: Props) => {
    const { nextInstruction, step } = props;


    return (
        <Card>
            <H6>Next Instruction</H6>
            {instrucitonDisplay(nextInstruction)}
            <Button onClick={step}>EXECUTE</Button>
        </Card>
    )
}

export default NextInstruction;