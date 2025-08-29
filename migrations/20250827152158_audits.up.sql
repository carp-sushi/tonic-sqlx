create table audits (
  id uuid default gen_random_uuid() primary key,
  audit_actor text not null,
  audit_action text not null,
  audit_location text not null,
  audit_ts timestamptz not null default now()
);

select set_undeleteable_table('audits');

select set_immutable_columns(
  'audits', 'id', 'audit_actor', 'audit_action', 'audit_location', 'audit_ts'
);
