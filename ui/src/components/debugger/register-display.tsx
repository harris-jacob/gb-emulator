import { h } from "preact";
import { RegisterView } from "../../emulator/registers";
import Button from "../generic/button";
import Card from "../generic/card";
import H6 from "../generic/H6";




/** Degubber component that displays current register values */
const RegisterDisplay = ({ register }: { register: RegisterView }): JSX.Element => {

    return (
        <Card>
            <H6>Register Values</H6>
            {`AF: ${register.af()}`} <br />
            {`BC: ${register.bc()}`} <br />
            {`HL: ${register.hl()}`} <br />
            {`SP: ${register.sp()}`} <br />
            {`PC: ${register.pc()}`} <br />
        </Card>
    )
}

export default RegisterDisplay;