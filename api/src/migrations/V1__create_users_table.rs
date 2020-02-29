use barrel::{
    backend::Sqlite,
    migration::Migration,
    types::{date, primary, varchar},
};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table_if_not_exists("users", |t| {
        t.add_column("id", primary());
        t.add_column("email", varchar(255).unique(true));
        t.add_column("created_at", date().default("current_timestamp"));
        t.add_column("updated_at", date().nullable(true));
    });

    m.make::<Sqlite>()
}
