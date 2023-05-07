# Tyst
A DSL to write programs using just `Ty`pes in Ru`st`.

### Example
The code
```rust
func!(
  (AddCustom (A:1 B:2) (+ (get 1) (get 2)))
);

func!(
  (Factorial (N:1)
    ((set 2 1)
     (range 3 1 (AddCustom (get 1) 1)
       (set 2 (* (get 3) (get 2))))
     (get 2)))
);

type Program = computation!(
  (set 1 (AddCustom 6 5))
  (if (= (AddCustom 6 5) (Add 5 6))
    (set 1 (Factorial 5))
    (set 1 (Factorial 6)))
  (get 1)
);
```

Basically expands to
```rust
struct AddCustom<A, B>(std::marker::PhantomData<A>, std::marker::PhantomData<B>);
impl<A: $crate::Computation, B: $crate::Computation> $crate::Computation for AddCustom<A, B> {
    fn compute(vars: &mut std::collections::HashMap<usize, isize>) -> isize {
        let mut new_map = std::collections::HashMap::<usize, isize>::new();
        new_map.insert(1, A::compute(vars));
        new_map.insert(2, B::compute(vars));
        type Exec = $crate::Add<$crate::GetVar<1>, $crate::GetVar<2>>;
        Exec::compute(&mut new_map)
    }
}

struct Factorial<N>(std::marker::PhantomData<N>);
impl<N: $crate::Computation> $crate::Computation for Factorial<N> {
    fn compute(vars: &mut std::collections::HashMap<usize, isize>) -> isize {
        let mut new_map = std::collections::HashMap::<usize, isize>::new();
        new_map.insert(1, N::compute(vars));
        type Exec = $crate::Seq<
            $crate::SetVar<2, $crate::Const<1>>,
            $crate::Seq<
                $crate::Range<
                    3,
                    $crate::Const<1>,
                    AddCustom<$crate::GetVar<1>, $crate::Const<1>>,
                    $crate::SetVar<2, $crate::Mul<$crate::GetVar<3>, $crate::GetVar<2>>>,
                >,
                $crate::GetVar<2>,
            >,
        >;
        Exec::compute(&mut new_map)
    }
}

type Program = $crate::Seq<
    $crate::SetVar<1, AddCustom<$crate::Const<6>, $crate::Const<5>>>,
    $crate::Seq<
        $crate::IfElse<
            $crate::Equ<
                AddCustom<$crate::Const<6>, $crate::Const<5>>,
                Add<$crate::Const<5>, $crate::Const<6>>,
            >,
            $crate::SetVar<1, Factorial<$crate::Const<5>>>,
            $crate::SetVar<1, Factorial<$crate::Const<6>>>,
        >,
        $crate::GetVar<1>,
    >,
>;
```
Which is a program made up of only types that will execute and produce the correct output. Very cool and very cursed.
