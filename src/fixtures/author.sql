with author as (
	insert into author (last_name)
	values ('Singh')
	returning id
)
update book
set author_id = (select id from author)
where title = 'How To Do Nothing';

with author as (
	insert into author (first_name, last_name)
	values ('Fork', 'Parker')
	returning id
)
update book
set author_id = (select id from author)
where title = 'How To Maximize Profits';
