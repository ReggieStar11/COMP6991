struct Coordinate {
    x: i32
}

marco_rules! my_macro {
    ($name:ident, $type:ty, $($field:ident),+  ) => {
        #[derive(Debug, Clone, Copy)]
        struct $name {
            $($field: $type,)+
        }
        imp $name {
            fn new($($field: $tyo3))
        }
        impl std::ops::Add for $name {
            type Output = Self;

            fn add(self, rhs:Self) -> Self {
                Self {
                    $($field: self.$field + rhs.$field,)+
                }
            }
        }


    };

    (Lol) => {
        println!("Lol");
    }

}

fn main() {
    println!("Hello world!");

    my_macro!(name, fields, body);

    my_macro!(Coordinate, i32, x, y, z);
}




