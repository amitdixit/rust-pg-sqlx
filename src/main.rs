// #[allow(unused_imports)]
// #[allow(unused)]
//#[allow(dead_code, unused, unused_imports)]

#![allow(unused)]
// use futures_util::stream::TryStreamExt;
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{prelude::FromRow, query, Row};
use std::error::Error;

#[derive(Debug, FromRow)] //FromRow allows to read data directly as typed so it removed the iteration
struct Book {
    title: String,
    author: String,
    isbn: String,
}

#[derive(Debug, FromRow)]
struct BookJson {
    title: String,
    metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    pub avg_review: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, FromRow)] //FromRow allows to read data directly as typed so it removed the iteration
struct BookUUID {
    pub id: uuid::Uuid,
    pub title: String,
}

#[derive(Debug, FromRow)] //FromRow allows to read data directly as typed so it removed the iteration
struct BookChrono {
    title: String,
    published_date: chrono::NaiveDate,
    inserted_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn create(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "insert into book (title,author,isbn) values($1,$2,$3)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

async fn update(book: &Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "update book set title=$1,author=$2 where isbn=$3";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.isbn)
        .execute(pool)
        .await?;

    Ok(())
}

//fetch_one returns Error: RowNotFound if No records
async fn read_one(pool: &sqlx::PgPool) -> Result<Book, Box<dyn Error>> {
    let query = "SELECT isbn, title, author FROM public.book where isbn='134'";
    let row = sqlx::query(query).fetch_one(pool).await?;

    let book = Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    };

    Ok(book)
}

//fetch_optional returns Doesnt Returns Error if No records
async fn read_one_optional(pool: &sqlx::PgPool) -> Result<Option<Book>, Box<dyn Error>> {
    let query = "SELECT isbn, title, author FROM public.book where isbn='134'";
    let maybe_row = sqlx::query(query).fetch_optional(pool).await?;

    let book = maybe_row.map(|row| Book {
        title: row.get("title"),
        author: row.get("author"),
        isbn: row.get("isbn"),
    });

    Ok(book)
}

async fn read_all(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let query = "SELECT isbn, title, author FROM public.book";
    let rows = sqlx::query(query).fetch_all(pool).await?;

    let books = rows
        .iter()
        .map(|row| Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        })
        .collect();

    Ok(books)
}

async fn read_all_stream(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let query = "SELECT isbn, title, author FROM public.book";
    let mut rows = sqlx::query(query).fetch(pool);

    let mut books = vec![];

    while let Some(row) = rows.try_next().await? {
        books.push(Book {
            title: row.get("title"),
            author: row.get("author"),
            isbn: row.get("isbn"),
        });
        //  books.push(book);
    }

    Ok(books)
}

async fn read_all_typed(pool: &sqlx::PgPool) -> Result<Vec<Book>, Box<dyn Error>> {
    let q = "SELECT isbn, title, author FROM public.book";

    let query = sqlx::query_as::<_, Book>(q);

    let books = query.fetch_all(pool).await?;

    Ok(books)
}

async fn insert_book(book: Book, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let mut txn = pool.begin().await?;

    let author_q = r"
    insert into author (name) values($1) returning id
    ";

    let book_q = "insert into book1 (title, author_id,isbn) values($1, $2,$3)";

    let author_id: (i64,) = sqlx::query_as(author_q)
        .bind(&book.author)
        .fetch_one(&mut *txn)
        .await?;

    sqlx::query(book_q)
        .bind(&book.title)
        .bind(&author_id.0)
        .bind(&book.isbn)
        .execute(&mut *txn)
        .await?;

    txn.commit().await?;

    Ok(())
}

async fn insert_json(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let book = BookJson {
        title: "A Game of Thrones".to_string(),
        metadata: Metadata {
            avg_review: 9.0,
            tags: vec!["Fantasy".to_string(), "Fiction".to_string()],
        },
    };

    let query = "insert into book_json (title,metadata) values($1,$2)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(Json(&book.metadata))
        .execute(pool)
        .await?;

    Ok(())
}

async fn insert_uuid(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let book = BookUUID {
        title: "A Game of Thrones".to_string(),
        id: uuid::Uuid::parse_str("21450598-bc9f-4532-8a25-b914a0380174")?,
    };

    let query = "insert into book_uuid (id,title) values($1,$2)";

    sqlx::query(query)
        .bind(&book.id)
        .bind(&book.title)
        .execute(pool)
        .await?;

    let q = " select id,title from book_uuid where id = $1";
    let res = sqlx::query_as::<_, BookUUID>(q)
        .bind(&book.id)
        .fetch_one(pool)
        .await?;

    println!("{:?}", res);

    Ok(())
}

async fn insert_chrono(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let book = BookChrono {
        title: "A Game of Thrones".to_string(),
        published_date: chrono::NaiveDate::from_ymd_opt(1996, 11, 8).unwrap(),
        inserted_at: None,
    };

    let query = "insert into book_chrono (title,published_date) values($1,$2)";

    sqlx::query(query)
        .bind(&book.title)
        .bind(&book.published_date)
        .execute(pool)
        .await?;

    let q = " select title,published_date,inserted_at from book_chrono";
    let res = sqlx::query_as::<_, BookChrono>(q).fetch_one(pool).await?;

    println!("{:?}", res);

    Ok(())
}

//alwaus needs .env file  with DATABASE_URL=postgres://PG_USER:PG_PASSWORD@HOST_NAME:5432/appdb
async fn read_queryas(isbn: &str, pool: &sqlx::PgPool) -> Result<Option<Book>, Box<dyn Error>> {
    let book = sqlx::query_as!(
        Book,
        "select b.title ,a.name as author ,b.isbn  from book1 b join author a  on a.id =b.author_id   where b.isbn=$1",
        isbn
    )
    .fetch_optional(pool)
    .await?;

    Ok(book)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "postgres://PG_USER:PG_PASSWORD@HOST_NAME:5432/appdb";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    // region: Insert
    /*
    let book = Book {
        title: "The Hobbit".to_string(),
        author: "J.R.R. Tolkien".to_string(),
        isbn: "978-0547928227".to_string(),
    };

    let book = Book {
        title: "The Da Vinci Code".to_string(),
        author: "Dan Brown".to_string(),
        isbn: "978-0486282114".to_string(),
    };

    let book = Book {
        title: "Ramayan".to_string(),
        author: "Valmiki".to_string(),
        isbn: "978-0486282115".to_string(),
    };

    let book = Book {
        title: "Mahabharat".to_string(),
        author: "Ved Vyas".to_string(),
        isbn: "958-0486282115".to_string(),
    };

    let book = Book {
        title: "Mr. X in Bombay".to_string(),
        author: "Mr. Y".to_string(),
        isbn: "958-048X282115".to_string(),
    };

    create(&book, &pool).await?;
    */
    // endregion

    // region: Update
    /*
        let book = Book {
            title: "Mr. X in Bombay".to_string(),
            author: "Mr. X".to_string(),
            isbn: "958-048X282115".to_string(),
        };

        update(&book, &pool).await?;
    */
    // endregion

    // region: read_one
    /*
        let fetch_one = read_one(&pool).await?;

        println!("{:#?}", fetch_one);
    */
    // endregion

    // region: read_one_optional
    /*
    let fetch_optional = read_one_optional(&pool).await?;

    println!("{:#?}", fetch_optional);

    */
    // endregion

    // region: read_all
    /* let fetch_all = read_all(&pool).await?;
    println!("{:#?}", fetch_all);
    */
    // endregion

    // region: read_stream
    /*
    let fetch_stream = read_all_stream(&pool).await?;
    println!("{:#?}", fetch_stream);
    */
    // endregion

    // region: read_typed

    /*
    let fetch_typed = read_all_typed(&pool).await?;
    println!("{:#?}", fetch_typed);
    */
    // endregion

    // region: insert_book transaction

    /*
     let book = Book {
        title: "The Testament".to_string(),
        author: "John Grisham".to_string(),
        isbn: "998-048X282115".to_string(),
    };

    insert_book(book, &pool).await?; */

    // endregion

    // region: insert_json
    /* insert_json(&pool).await?;*/
    // endregion

    // insert_uuid(&pool).await?;
    // insert_chrono(&pool).await?;

    let book = read_queryas("998-048X282115", &pool).await?;
    println!("{:#?}", book);

    println!("Execution Finished");

    Ok(())
}
