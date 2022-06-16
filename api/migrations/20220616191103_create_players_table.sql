create table "players" (
    "id" uuid primary key default uuid_generate_v1mc(),
    "name" varchar(255) not null,
    "score" integer not null,
    "created_at" timestamp not null default current_timestamp
);