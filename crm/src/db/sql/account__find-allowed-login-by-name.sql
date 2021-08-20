select *
from account
where name = $1
  and allow_login = true