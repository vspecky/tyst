use std::{collections::HashMap, marker::PhantomData};

pub trait Computation {
    fn compute(vars: &mut HashMap<usize, isize>) -> isize;
}

pub struct Const<const VAL: isize>;
pub struct SetVar<const VAR: usize, C: Computation>(PhantomData<C>);
pub struct GetVar<const VAR: usize>;
pub struct And<L: Computation, R: Computation>(PhantomData<(L, R)>);
pub struct Or<L: Computation, R: Computation>(PhantomData<(L, R)>);
pub struct Seq<C: Computation, N: Computation>(PhantomData<(C, N)>);
pub struct IfElse<Cond: Computation, Then: Computation, Else: Computation>(
    PhantomData<(Cond, Then, Else)>,
);
pub struct Range<const VAR: usize, From: Computation, To: Computation, Do: Computation>(
    PhantomData<(From, To, Do)>,
);

impl<const VAL: isize> Computation for Const<VAL> {
    fn compute(_vars: &mut HashMap<usize, isize>) -> isize {
        VAL
    }
}

impl<const VAR: usize> Computation for GetVar<VAR> {
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        vars.get(&VAR).copied().unwrap_or_default()
    }
}

impl<const VAR: usize, C> Computation for SetVar<VAR, C>
where
    C: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        let res = C::compute(vars);
        vars.insert(VAR, res);
        res
    }
}

impl<C, N> Computation for Seq<C, N>
where
    C: Computation,
    N: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        C::compute(vars);
        N::compute(vars)
    }
}

macro_rules! bin_impl {
    ($(($name:ident $op:tt))+) => {
        $(
            pub struct $name<L: Computation, R: Computation>(PhantomData<(L, R)>);

            impl<L, R> Computation for $name<L, R>
            where
                L: Computation,
                R: Computation,
            {
                fn compute(vars: &mut HashMap<usize, isize>) -> isize {
                    L::compute(vars) $op R::compute(vars)
                }
            }
        )+
    };
}

bin_impl!(
    (Add +)
    (Sub -)
    (Mul *)
    (Div /)
    (BitAnd &)
    (BitOr |)
);

macro_rules! cond_impl {
    ($(($name:ident $op:tt))+) => {
        $(
            pub struct $name<L: Computation, R: Computation>(PhantomData<(L, R)>);

            impl<L, R> Computation for $name<L, R>
            where
                L: Computation,
                R: Computation,
            {
                fn compute(vars: &mut HashMap<usize, isize>) -> isize {
                    if L::compute(vars) $op R::compute(vars) {
                        1
                    } else {
                        0
                    }
                }
            }
        )+
    };
}

cond_impl!(
    (Gt >)
    (Lt <)
    (Gte >=)
    (Lte <=)
    (Equ ==)
);

impl<L, R> Computation for And<L, R>
where
    L: Computation,
    R: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        if L::compute(vars) > 0 && R::compute(vars) > 0 {
            1
        } else {
            0
        }
    }
}

impl<L, R> Computation for Or<L, R>
where
    L: Computation,
    R: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        if L::compute(vars) > 0 || R::compute(vars) > 0 {
            1
        } else {
            0
        }
    }
}

impl<Cond, Then, Else> Computation for IfElse<Cond, Then, Else>
where
    Cond: Computation,
    Then: Computation,
    Else: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        if Cond::compute(vars) > 0 {
            Then::compute(vars)
        } else {
            Else::compute(vars)
        }
    }
}

impl<const VAR: usize, From, To, Do> Computation for Range<VAR, From, To, Do>
where
    From: Computation,
    To: Computation,
    Do: Computation,
{
    fn compute(vars: &mut HashMap<usize, isize>) -> isize {
        let from = From::compute(vars);
        let to = To::compute(vars);
        for i in from..to {
            vars.insert(VAR, i);
            Do::compute(vars);
        }
        0
    }
}

#[macro_export]
macro_rules! seq_body {
    (($(($($op:tt)+))+)) => {
        $crate::computation!($(($($op)+))+)
    };

    (($($op:tt)+)) => {
        $crate::op!($($op)+)
    };
}

#[macro_export]
macro_rules! arg_expand {
    (($($op:tt)+)) => {
        $crate::op!($($op)+)
    };

    ($op:tt) => {
        $crate::op!($op)
    };
}

#[macro_export]
macro_rules! op {
    (set $var:literal $inner:tt) => {
        $crate::SetVar<$var, $crate::arg_expand!($inner)>
    };

    (get $var:literal) => {
        $crate::GetVar<$var>
    };

    (if $cond:tt $then:tt $else:tt) => {
        $crate::IfElse<
            $crate::arg_expand!($cond),
            $crate::seq_body!($then),
            $crate::seq_body!($else)
        >
    };

    (+ $lhs:tt $rhs:tt) => {
        $crate::Add<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (- $lhs:tt $rhs:tt) => {
        $crate::Sub<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (* $lhs:tt $rhs:tt) => {
        $crate::Mul<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (/ $lhs:tt $rhs:tt) => {
        $crate::Div<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (& $lhs:tt $rhs:tt) => {
        $crate::BitAnd<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (| $lhs:tt $rhs:tt) => {
        $crate::BitOr<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (> $lhs:tt $rhs:tt) => {
        $crate::Gt<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (>= $lhs:tt $rhs:tt) => {
        $crate::Gte<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (< $lhs:tt $rhs:tt) => {
        $crate::Lt<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (<= $lhs:tt $rhs:tt) => {
        $crate::Lte<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (= $lhs:tt $rhs:tt) => {
        $crate::Equ<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (&& $lhs:tt $rhs:tt) => {
        $crate::And<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (|| $lhs:tt $rhs:tt) => {
        $crate::Or<$crate::arg_expand!($lhs), $crate::arg_expand!($rhs)>
    };

    (range $var:literal $from:tt $to:tt $body:tt) => {
        $crate::Range<
            $var,
            $crate::arg_expand!($from),
            $crate::arg_expand!($to),
            $crate::seq_body!($body)
        >
    };

    ($func:ident $($arg:tt)*) => {
        $func<$($crate::arg_expand!($arg)),*>
    };

    ($i:literal) => {
        $crate::Const<$i>
    };
}

#[macro_export]
macro_rules! computation {
    (($($op:tt)+)) => {
        op!($($op)+)
    };

    (($($op:tt)+) $(($($n_op:tt)+))+) => {
        $crate::Seq<op!($($op)+), $crate::computation!($(($($n_op)+))+)>
    };
}

#[macro_export]
macro_rules! func {
    (($vis:vis $name:ident ($($arg:ident:$var:literal)*) $body:tt)) => {
        $vis struct $name<$($arg),*>($(std::marker::PhantomData<$arg>),+);

        impl<$($arg: $crate::Computation),*> $crate::Computation for $name<$($arg),*> {
            fn compute(vars: &mut std::collections::HashMap<usize, isize>) -> isize {
                let mut new_map = std::collections::HashMap::<usize, isize>::new();
                $(
                    new_map.insert($var, $arg::compute(vars));
                )*

                type Exec = $crate::seq_body!($body);

                Exec::compute(&mut new_map)
            }
        }
    };
}

pub fn run<T: Computation>() -> isize {
    let mut map: HashMap<usize, isize> = HashMap::new();
    T::compute(&mut map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
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
        assert_eq!(run::<Program>(), 120);
    }

    #[test]
    fn test_fibonacci() {
        func!(
          (Fibonacci (N:1)
            (if (< (get 1) 2)
              (1)
              (+ (Fibonacci (- (get 1) 1)) (Fibonacci (- (get 1) 2)))))
        );

        type Program = op!(Fibonacci 10);
        assert_eq!(run::<Program>(), 89);
    }
}
