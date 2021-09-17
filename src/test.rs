macro_rules! my_struct {
    ($p:vis $name:ident) => {
        $p struct $name;
    };
}

my_struct!(A);