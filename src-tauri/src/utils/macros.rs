#[macro_export]
macro_rules! insert {
    ( $table:expr) => {{
        println!("inserting a new tupel");
        diesel::insert_into($table)
    }};
}

#[macro_export]
macro_rules! update {
    ( $table:expr,$predicate:expr) => {{
        println!("update a tupel");
        diesel::update($table.filter($predicate))
    }};
}

#[macro_export]
macro_rules! delete {
    ( $table:expr, $predicate:expr) => {{
        println!("deleting a tupel");
        diesel::delete($table.filter($predicate))
    }};
}
#[macro_export]
macro_rules! deleteAll {
    ( $table:expr) => {{
        println!("deleting all tupel");
        diesel::delete($table)
    }};
}
#[macro_export]
macro_rules! connect {
    ($store: expr) => {{
        &mut $store
            .get()
            .map_err(|e| panic!("Connection not found. {}", e))
            .unwrap()
    }};
}
