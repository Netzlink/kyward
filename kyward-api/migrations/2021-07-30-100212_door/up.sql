-- Your SQL goes here
create table `doors` (
    `id` integer not null,
    `name` char(32) not null,
    `compartment` char(32) not null,
    `level` char(16) not null,
    `building` char(48) not null,
    `description` text not null,
    primary key (`id`)
);
create table `groups` (
    `id` integer not null,
    `name` char(16) not null,
    `door_id` integer not null,
    `description` text not null,
    primary key (`id`, `door_id`),
    foreign key (`door_id`) references `doors` (`id`)
);
create table `companies` (
    `id` integer not null,
    `name` char(32) not null,
    `description` text not null,
    primary key (`id`)
);
create table `tokens` (
    `id` integer not null,
    `value` char(8) not null,
    `reverse` char(8) not null,
    `description` text not null,
    primary key (`id`)
);
create table `persons` (
    `id` integer not null,
    `first_name` char(32) not null,
    `last_name` char(32) not null,
    `ema` text not null,
    `enabled` boolean not null default true,
    `company_id` integer not null,
    `token_id` integer not null,
    `group_id` integer not null,
    `description` text not null,
    primary key (`id`, `group_id`),
    foreign key (`company_id`) references `company` (`id`),
    foreign key (`token_id`) references `token` (`id`),
    foreign key (`group_id`) references `group` (`id`)
);