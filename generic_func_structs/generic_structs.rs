struct S<T1, T2> {
    c: char,
    n1: T1,
    n2: T1,
    n3: T2,
}
let _s = S { c: 'a', n1: 34, n2: 782, n3: 0.02 };
struct TS<T1, T2>(char, T1, T1, T2);
let _ts = TS('a', 34, 782, 0.02);
