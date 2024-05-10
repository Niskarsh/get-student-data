pub fn last_session_date(email: &str) -> String {
    format!(
        "
select
cb.time_scheduled
from
applications a
join cohorts c on a.cohort_joining = c.id
join cohort_breakouts cb on c.id = cb.cohort_id
join learner_breakouts lb on lb.cohort_breakout_id = cb.id
and lb.application_id = a.id
where
a.user_id in (
select
  u.id
from
  users u
where
u.email ilike '%{}%'
)
order by
cb.time_scheduled DESC
limit
1;
",
        email
    )
}

pub fn date_of_admission(email: &str) -> String {
    format!(
        "
select
c.start_date as \"Date of admission\"
from
applications a
join cohorts c on a.cohort_applied = c.id
where
a.user_id in (
select
  u.id
from
  users u
where
  u.email ilike '%{}%'
)
",
        email
    )
}

pub fn total_scheduled_sessions(email: &str) -> String {
    format!("
    select
  count(lb.*)
from
  applications a
  join learner_breakouts lb on lb.application_id = a.id 
where
  a.user_id in (
    select
      u.id
    from
      users u
    where
    u.email ilike '%{}%'
  )

    ", email)
}


pub fn total_attended_sessions(email: &str) -> String {
    format!("
    select
  count(lb.*)
from
  applications a
  join learner_breakouts lb on lb.application_id = a.id 
   and lb.attendance=true 
where
  a.user_id in (
    select
      u.id
    from
      users u
    where
    u.email ilike '%{}%'
  )

    ", email)
}