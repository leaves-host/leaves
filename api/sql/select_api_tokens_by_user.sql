select
  id,
  contents,
  user_id
from
  api_tokens
where
  user_id = ?1
