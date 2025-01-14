use crate::tests::test_builder::test_builder;
use expect_test::expect;

#[test]
fn memory_size_alignment_issue() {
    test_builder()
        .wat(
            r#"
              (module
                (type (;0;) (func))
                (func (;0;) (type 0)
                  memory.size
                  drop
                )
                (memory (;0;) 1 1024)
                (export "foo" (func 0))
              )
            "#,
        )
        .method("foo")
        .expects(&[
            expect![[r#"
                VMOutcome: balance 4 storage_usage 12 return data None burnt gas 46411725 used gas 46411725
            "#]],
        ]);
}
