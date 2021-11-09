import { FunctionalComponent, h } from "preact";
import { AutoSizer, Column, Size, Table } from "react-virtualized";
import { Instruction } from "../../emulator/types";
import Card from "../generic/card";
import 'react-virtualized/styles.css'
import theme from "../../theme";

interface Props {
    instructions: Instruction[];
    pc: number;
}

// handle translation from PC to table index
const pcToIndex = (pc: number) => pc - 0x100;


const InstructionList: FunctionalComponent<Props> = (props: Props) => {
    const { instructions, pc } = props;

    const rowStyler = ({ index }: { index: number }) => {
        if (index === pcToIndex(pc)) {
            return { backgroundColor: theme.palette.attention }
        }
    }

    return (
        <Card>
            <AutoSizer>
                {({ width, height }: Size) => (
                    <Table
                        height={height}
                        rowCount={instructions.length}
                        rowHeight={30}
                        headerHeight={20}
                        rowGetter={({ index }: { index: number }) => instructions[index]}
                        width={width}
                        rowStyle={rowStyler}
                        scrollToIndex={pcToIndex(pc)}
                    >
                        <Column label="Address" dataKey="address" width={width / 4} />
                        <Column label="Opcode" dataKey="opcode" width={width / 4} />
                        <Column label="Name" dataKey="name" width={width / 4} />
                        <Column label="Operand" dataKey="operand" width={width / 4} />
                    </Table>
                )}
            </AutoSizer>
        </Card>
    )
}

export default InstructionList;