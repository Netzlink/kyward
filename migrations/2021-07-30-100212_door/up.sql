-- Your SQL goes here
create table `doors` (
    `id` integer not null primary key,
    `name` char(32) not null,
    `compartment` char(32) not null,
    `level` char(16) not null,
    `building` char(48) not null,
    `description` text not null
);