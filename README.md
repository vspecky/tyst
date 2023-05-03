# Tyst
A DSL to write programs using just `Ty`pes in Ru`st`.

### Example
The code
```rust
func!(
  (Add (A:1 B:2)
    ((+ (#1) (#2))))
);

func!(
  (Factorial (N:1)
    ((set 2 (1))
     (range 3 (1) (Add (#1) (1))
       ((set 2 (* (#3) (#2)))))
     (#2)))
);

type Program = computation!(
  (set 1 (Add (6) (5)))
  (if (= (Add (6) (5)) (Add (5) (6)))
    ((set 1 (Factorial (5))))
    ((set 1 (Factorial (6)))))
  (#1)
);
```

Basically expands to
```rust
struct Add<A, B>(std::marker::PhantomData<A>, std::marker::PhantomData<B>);
impl<A: Computation, B: Computation> Computation for Add<A, B> {
    fn compute(vars: &mut std::collections::HashMap<usize, isize>) -> isize {
        let mut new_map = std::collections::HashMap::<usize, isize>::new();
        new_map.insert(1, A::compute(vars));
        new_map.insert(2, B::compute(vars));
        type Exec = Add<GetVar<1>, GetVar<2>>;
        Exec::compute(&mut new_map)
    }
}

struct Factorial<N>(std::marker::PhantomData<N>);
impl<N: Computation> Computation for Factorial<N> {
    fn compute(vars: &mut std::collections::HashMap<usize, isize>) -> isize {
        let mut new_map = std::collections::HashMap::<usize, isize>::new();
        new_map.insert(1, N::compute(vars));
        type Exec = Seq<
            SetVar<2, Const<1>>,
            Seq<
                Range<
                    3,
                    Const<1>,
                    Add<GetVar<1>, Const<1>>,
                    SetVar<2, Mul<GetVar<3>, GetVar<2>>>,
                >,
                GetVar<2>,
            >,
        >;
        Exec::compute(&mut new_map)
    }
}


type Program = Seq<
    SetVar<1, Add<Const<6>, Const<5>>>,
    Seq<
        IfElse<
            Equ<
                Add<Const<6>, Const<5>>,
                Add<Const<5>, Const<6>>,
            >,
            SetVar<1, Factorial<Const<5>>>,
            SetVar<1, Factorial<Const<6>>>,
        >,
        GetVar<1>,
    >,
>
```
Which is a program made up of only types that will execute and produce the correct output. Very cool and very cursed.
