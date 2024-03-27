create table book (
	id integer primary key generated always as identity,
	title text not null,
	year integer
);
