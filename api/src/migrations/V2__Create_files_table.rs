use barrel::{
    backend::Sqlite,
    migration::Migration,
    types::{date, foreign, integer, varchar},
};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table_if_not_exists("files", |t| {
        t.add_column("id", varchar(20).unique(true).indexed(true));
        t.add_column("size", integer());
        t.add_column("user_id", foreign("users", "id"));
        t.add_column("created_at", date().default("current_timestamp"));
        t.add_column("updated_at", date().nullable(true));
    });

    m.make::<Sqlite>()
}
