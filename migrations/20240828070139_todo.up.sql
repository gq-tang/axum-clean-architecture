-- Add up migration script here
create table
    todos (
        id integer not null primary key AUTOINCREMENT,
        user_id integer not null,
        title varchar not null,
        description text not null,
        completed boolean not null DEFAULT false,
        created_at timestamp not null default CURRENT_TIMESTAMP,
        updated_at timestamp not null default CURRENT_TIMESTAMP
    );