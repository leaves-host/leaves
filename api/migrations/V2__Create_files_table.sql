create table if not exists files (
  id text primary key not null unique,
  size int not null,
  user_id bigint not null references users (id),
  created_at timestamp not null default now()
);
