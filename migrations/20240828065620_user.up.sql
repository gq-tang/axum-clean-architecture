-- Add up migration script here
create table
    users (
        id integer not null primary key AUTOINCREMENT,
        user_name varchar(50) not NULL,
        nick_name varchar(255) not NULL,
        password varchar(255) not null,
        created_at timestamp not null default CURRENT_TIMESTAMP,
        updated_at timestamp not null DEFAULT CURRENT_TIMESTAMP
    );

create index idx_user_user_name on users (user_name);