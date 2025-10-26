#[macro_export]
macro_rules! map {
    ( $( $x:expr => $y:expr ), * $(,)? ) => {
        {
            let mut temp_hashmap = hashbrown::HashMap::new();
            $(
                temp_hashmap.insert($x.to_string(), $y);
            )*
            temp_hashmap
        }
    };
}
