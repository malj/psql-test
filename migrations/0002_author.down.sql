alter table book drop constraint book_author_id_fkey;

alter table book drop column author_id;

drop table author;
