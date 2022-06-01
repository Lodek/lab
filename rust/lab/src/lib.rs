
mod boxed_trait_obj_with_asociated_type {
    use std::any::Any;

    trait Thing {
        type Foo;
        fn act(&self, param: Self::Foo);
    }

    struct Uno {}
    impl Thing for Uno {
        type Foo = i32;
        fn act(&self, foo: i32) { }
    }
    
    struct Dos { }
    impl Thing for Dos {
        type Foo = char;
        fn act(&self, foo: char) { }
    }

    // how do I make struct member capable of storing both Uno and Dos?
    struct Boxer {
        thing: Box<dyn Thing<Foo = _>>
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test() {
            let uno = Uno{};
            let dos = Dos{};
            let mut boxer = Boxer { thing: Box::new(uno) };
            boxer.thing.act();
            // do stuff then
            boxer.thing = Box::new(dos);
            boxer.thing.act();
        }
    }
}
