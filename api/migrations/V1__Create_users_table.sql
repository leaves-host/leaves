create table if not exists users (
  id bigserial primary key not null,
  email text not null unique,
  created_at timestamp not null default now(),
  updated_at timestamp
);
