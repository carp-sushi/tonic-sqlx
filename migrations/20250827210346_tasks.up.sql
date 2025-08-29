create table tasks (
    id uuid default gen_random_uuid() primary key,
    story_id uuid references stories(id) not null,
    name text not null,
    status text not null default 'incomplete'
);

create index tasks_story_id_index ON tasks using btree(story_id);

select add_timestamp_columns('tasks');

select set_immutable_columns('tasks', 'id', 'story_id', 'created_at');