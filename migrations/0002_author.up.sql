create table author (
	id integer primary key generated always as identity,
	first_name text,
	last_name text not null
);

alter table book add column author_id integer;

alter table book add constraint book_author_id_fkey
foreign key (author_id) references author (id);
