create table stories (
  id uuid default gen_random_uuid() primary key,
  name text not null,
  seqno bigint generated always as identity
);

create index stories_seqno_index on stories using btree(seqno);

select add_timestamp_columns('stories');

select set_immutable_columns('stories', 'id', 'created_at');
