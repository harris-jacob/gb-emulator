import { h } from "preact";
import styled from "styled-components";
import { RegisterView } from "../../emulator/registers";
import theme from "../../theme";
import Card from "../generic/card";
import H6 from "../generic/H6";


const Table = styled.div`
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
`

const Column = styled.div`
    padding: ${theme.spacing(2)};
`

/** Degubber component that displays current register values */
const RegisterDisplay = ({ register }: { register: RegisterView }): JSX.Element => {

    const sixteenBit = (
        <Column>
            {`AF: ${register.af()}`} <br />
            {`BC: ${register.bc()}`} <br />
            {`HL: ${register.hl()}`} <br />
            {`SP: ${register.sp()}`} <br />
            {`PC: ${register.pc()}`} <br />
        </Column>
    )

    const eightBit = (
        <Column>
            {`A: ${register.a()}`} <br />
            {`B: ${register.b()}`} <br />
            {`C: ${register.c()}`} <br />
            {`H: ${register.h()}`} <br />
            {`L: ${register.l()}`} <br />
        </Column>
    )

    const flags = (
        <Column>
            {`Carry: ${register.isCarrySet()}`} <br />
            {`Half Carry: ${register.isHalfCarrySet()}`} <br />
            {`Subtract: ${register.isSubtractSet()}`} <br />
            {`Zero: ${register.isZeroSet()}`} <br />

        </Column>
    )


    return (
        <Card>
            <H6>Register Values</H6>
            <Table>
                {sixteenBit}
                {eightBit}
                {flags}
            </Table>
        </Card>
    )
}

export default RegisterDisplay;