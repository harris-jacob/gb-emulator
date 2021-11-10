/** Get value of nth bit of number */
export const getBit = (a: number, n: number): number => {
    return ((1 << n) & a) >> n;
}