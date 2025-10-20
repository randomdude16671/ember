#[macro_export]
macro_rules! map {
    ( $( $x:expr => $y:expr ), * $(,)? ) => {
        {
            let temp_hashmap = hashbrown::HashMap::new();
            $(
                temp_hashmap.insert($x, $y);
            )*
            temp_hashmap
        }
    };
}
