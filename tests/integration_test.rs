use things::{Query, Read, System, Things};

struct AssertValues;
impl<'a> System<'a> for AssertValues {
    type Query = (Read<i32>, Read<&'static str>);

    fn update(components: <Self::Query as Query<'a>>::Iter) {
        for (int, string) in components {
            assert_eq!(int, &10);
            assert_eq!(string, &"hello");
        }
    }
}

#[test]
fn test_reader_system() {
    let mut ecs = Things::new();
    ecs.create_entity((10, "hello"));
    ecs.execute_system::<AssertValues>();
}
