import { FunctionalComponent, h } from "preact";
import { useMemo } from "preact/hooks";
import { AutoSizer, List, ListRowProps, Size } from "react-virtualized";
import { Instruction } from "../../emulator/types";
import { Table } from "../../utils/types";

interface Props {
    instructions: Table<Instruction>;
}


const InstructionList: FunctionalComponent<Props> = (props: Props) => {
    const { instructions } = props;

    const list = useMemo<Instruction[]>(() => {
        return Object.keys(instructions).map((v: string) => instructions[v])
    }, [instructions])

    const rowRenderer = (props: ListRowProps) => {
        return (
            <div key={props.key}>
                {list[props.index].name}
            </div>
        )
    }


    return (
        <AutoSizer>
            {({ width, height }: Size) => (
                <List
                    height={height}
                    rowCount={list.length}
                    rowHeight={120}
                    rowRenderer={rowRenderer}
                    width={width}
                />
            )}
        </AutoSizer>
    )
}

export default InstructionList;