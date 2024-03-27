#[sqlx::test(fixtures("book", "author"))]
async fn books(pool: sqlx::PgPool) -> sqlx::Result<()> {
	let books = sqlx::query!(
		"
		select
			book.id,
			book.title,
			book.year,
			author.first_name as author_first_name,
			author.last_name as author_last_name
		from book
		left join author on book.author_id = author.id;
		"
	)
	.fetch_all(&pool)
	.await?;

	assert_eq!(books.len(), 2);

	assert_eq!(books[0].id, 1);
	assert_eq!(books[0].title, "How To Do Nothing".to_string());
	assert_eq!(books[0].year, None);
	assert_eq!(books[0].author_first_name, None);
	assert_eq!(books[0].author_last_name, Some("Singh".to_string()));

	assert_eq!(books[1].id, 2);
	assert_eq!(books[1].title, "How To Maximize Profits".to_string());
	assert_eq!(books[1].year, Some(2009));
	assert_eq!(books[1].author_first_name, Some("Fork".to_string()));
	assert_eq!(books[1].author_last_name, Some("Parker".to_string()));

	Ok(())
}
