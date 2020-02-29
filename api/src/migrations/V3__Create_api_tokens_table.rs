use barrel::{
    backend::Sqlite,
    migration::Migration,
    types::{date, foreign, primary, varchar},
};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table_if_not_exists("api_tokens", |t| {
        t.add_column("id", primary());
        t.add_column("contents", varchar(50).unique(true));
        t.add_column("user_id", foreign("users", "id"));
        t.add_column("created_at", date().default("current_timestamp"));
    });

    m.make::<Sqlite>()
}
