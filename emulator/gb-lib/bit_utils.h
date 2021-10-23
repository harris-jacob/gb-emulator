#ifndef BIT_UTILS_H_
#define BIT_UTILS_H_

/* get bit n from int a */
#define GET_BIT(a, n) ((1 << n) & a) >> n;

/* set bit n from int a */
#define SET_BIT(a, n) a |= 1 << n;

/* clear bit n from int a */
#define CLEAR_BIT(a, n) a &= ~(1 << n)

#endif