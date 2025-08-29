--
-- Migration helper functions for creating timestamp columns
--

create or replace function
  create_timestamps()
  returns trigger as $$
begin
  new.created_at = now();
  new.updated_at = now();
  return new;
end $$
language plpgsql;

create or replace function
  update_timestamps()
  returns trigger as $$
begin
  new.updated_at = now();
  return new;
end $$
language plpgsql;

create or replace function
  add_timestamp_columns (table_name text)
  returns void as $$
begin
    execute format('ALTER TABLE %I ADD COLUMN IF NOT EXISTS created_at TIMESTAMPTZ NOT NULL;', table_name);
    execute format('ALTER TABLE %I ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL;', table_name);
    execute format('CREATE OR REPLACE TRIGGER %I_insert_ts BEFORE INSERT ON %I FOR EACH ROW EXECUTE PROCEDURE create_timestamps();', table_name, table_name);
    execute format('CREATE OR REPLACE TRIGGER %I_update_ts BEFORE UPDATE ON %I FOR EACH ROW EXECUTE PROCEDURE update_timestamps();', table_name, table_name);
end $$
language plpgsql;

--
-- Migration helper functions for making columns immutable.
--

create or replace function
  raise_immutability_exception()
  returns trigger as $$
begin
  raise exception 'This update was rejected because it attempted to UPDATE an immutable column. Old: %, New: %', OLD, NEW
  using hint = 'Tip: check the triggers on the table to see what columns are immutable';
end $$
language plpgsql;

create or replace function
  set_immutable_columns(variadic table_and_columns text[])
  returns void as $$
declare
  table_name text;
  columns text[];
  column_list text;
  column_condition text;
begin
  if array_length(table_and_columns, 1) < 2 then
    raise exception 'Pass at least one table and one column'
      using detail = format('Arguments received: %s', table_and_columns);
  end if;

  table_name := table_and_columns[1];
  columns := table_and_columns[2:];

  column_list := array_to_string(columns, ', ');
  column_condition := (select string_agg(x.y, ' OR ')
    from (select '(OLD.' || col || ' IS DISTINCT FROM NEW.' || col || ')' as y
    from unnest(columns) as col) as x);

  execute format(
    'CREATE OR REPLACE TRIGGER %s_immutable_columns
      AFTER UPDATE OF %s
      ON %s
      FOR EACH ROW
      WHEN (%s)
      EXECUTE FUNCTION raise_immutability_exception ();'
    , table_name
    , column_list
    , table_name
    , column_condition
    );
end $$
language plpgsql;

--
-- Migration helper functions for preventing DELETE on tables.
--

create or replace function
  raise_undeletable_table_exception()
  returns trigger as $$
begin
  raise exception 'DELETE is not allowed on this table';
end $$
language plpgsql;

create or replace function
  set_undeleteable_table (table_name text)
  returns void as $$
begin
    execute format('CREATE OR REPLACE TRIGGER %I_undeleteable_table AFTER DELETE ON %I FOR EACH STATEMENT EXECUTE FUNCTION raise_undeletable_table_exception();', table_name, table_name);
end $$
language plpgsql;

