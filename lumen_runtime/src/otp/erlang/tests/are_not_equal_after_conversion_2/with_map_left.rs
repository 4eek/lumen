use super::*;

#[test]
fn with_atom_right_returns_true() {
    are_not_equal_after_conversion(|_, _| Term::str_to_atom("right", DoNotCare).unwrap(), true)
}

#[test]
fn with_local_reference_right_returns_true() {
    are_not_equal_after_conversion(|_, mut process| Term::local_reference(&mut process), true);
}

#[test]
fn with_empty_list_right_returns_true() {
    are_not_equal_after_conversion(|_, _| Term::EMPTY_LIST, true);
}

#[test]
fn with_list_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| {
            Term::cons(
                0.into_process(&mut process),
                1.into_process(&mut process),
                &mut process,
            )
        },
        true,
    );
}

#[test]
fn with_small_integer_right_returns_true() {
    are_not_equal_after_conversion(|_, mut process| 0.into_process(&mut process), true)
}

#[test]
fn with_big_integer_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| (crate::integer::small::MAX + 1).into_process(&mut process),
        true,
    )
}

#[test]
fn with_float_right_returns_true() {
    are_not_equal_after_conversion(|_, mut process| 0.0.into_process(&mut process), true)
}

#[test]
fn with_local_pid_right_returns_true() {
    are_not_equal_after_conversion(|_, _| Term::local_pid(2, 3).unwrap(), true);
}

#[test]
fn with_external_pid_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| Term::external_pid(1, 2, 3, &mut process).unwrap(),
        true,
    );
}

#[test]
fn with_tuple_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| Term::slice_to_tuple(&[], &mut process),
        true,
    );
}

#[test]
fn with_same_map_right_returns_false() {
    are_not_equal_after_conversion(|left, _| left, false);
}

#[test]
fn with_same_value_map_right_returns_false() {
    are_not_equal_after_conversion(
        |_, mut process| Term::slice_to_map(&[], &mut process),
        false,
    );
}

#[test]
fn with_different_map_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| {
            Term::slice_to_map(
                &[(
                    Term::str_to_atom("a", DoNotCare).unwrap(),
                    1.into_process(&mut process),
                )],
                &mut process,
            )
        },
        true,
    );
}

#[test]
fn with_heap_binary_right_returns_true() {
    are_not_equal_after_conversion(
        |_, mut process| Term::slice_to_binary(&[], &mut process),
        true,
    );
}

#[test]
fn with_subbinary_right_returns_true() {
    are_not_equal_after_conversion(|_, mut process| bitstring!(1 :: 1, &mut process), true);
}

fn are_not_equal_after_conversion<R>(right: R, expected: bool)
where
    R: FnOnce(Term, &mut Process) -> Term,
{
    super::are_not_equal_after_conversion(
        |mut process| Term::slice_to_map(&[], &mut process),
        right,
        expected,
    );
}
