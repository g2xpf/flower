use flower_macros::flow;

flow! {
    resource: [
        A,
        B,
        C
    ]
    state: [P, Q, R, S]
    reference: [
        P -< mut A,
        Q -< B,
        R -< C,
    ]
    intermediate: [
        T
    ]
    transition: [
        A >--> B,
        B >- T -> C
    ]
    overlay: [
        R ^ S,
        Q ^ S
    ]
}

fn main() {}
