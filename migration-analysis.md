# Migration SQL Analysis

## 1. SQL Injection in `set_immutable_columns`

The column names in `set_immutable_columns` (init.up.sql:72-73) are concatenated directly without `%I` quoting. The `column_condition` and `column_list` are built via string concatenation, not `format('%I', ...)`. While inputs are controlled today, this is a latent injection vector.

## 2. `status` column should use a CHECK constraint or enum

`tasks.status` is `text not null default 'incomplete'` with no validation. A `CHECK (status IN ('incomplete', 'complete'))` constraint (or a Postgres enum type) would prevent invalid status values at the DB level.
