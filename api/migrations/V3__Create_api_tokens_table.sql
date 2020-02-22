create table if not exists api_tokens (
  id bigserial primary key not null,
  user_id bigint not null references users (id),
  contents text not null unique,
  created_at timestamp not null default now()
);
