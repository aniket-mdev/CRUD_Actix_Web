
extern crate dotenv;
use actix_web::Result;
use futures::stream::TryStreamExt; //add this



use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult}, Collection, Database
};

use crate::model::book_model::Book;



pub struct BookRepo {
    col:Collection<Book>,
}

impl BookRepo {
    pub async fn init(db:Database) -> Self {
        let col:Collection<Book> = db.collection("Book");
        BookRepo {col}
    }

    pub async fn create_book(&self, new_book:Book) -> Result<InsertOneResult, Error> {
        let new_doc = Book {
            id:None,
            book_name:new_book.book_name,
            book_author:new_book.book_author,
            total_page:new_book.total_page
        };

        let book = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating book");
        Ok(book)
    }

    pub async fn get_book(&self, id:ObjectId) -> Result<Book, Error> {
        
        let filter = doc! {"_id":id};
        let book = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("failed to fetched book");

        if book.is_none() {
            return Err(Error::DeserializationError { message: format!("book not found for this id") });
        }
        Ok(book.unwrap_or_default())
    }

    pub async fn get_books(&self) -> Result<Vec<Book>, Error> {
        let mut cursor = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error fecth books");
        
        let mut books:Vec<Book> = Vec::new();
        while let Some(book) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            books.push(book)    
        }
        Ok(books)

    }

    pub async fn update_book(&self, id:ObjectId, data:Book) -> Result<UpdateResult, Error> {
        let filter = doc! {"_id": id};
        let new_doc = doc! {
            "$set":{
                "book_name":data.book_name,
                "book_author":data.book_author,
                "total_page":data.total_page
            },
        };

        let update_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating books");
        Ok(update_doc)
    }

    pub async fn delete_book(&self, id:ObjectId) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id":id};

        let delete_book = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error delete book");

        Ok(delete_book)
    }

    pub async fn get_book_by_author(&self, id:String) -> Result<Vec<Book>, Error> {
        let filter = doc! {"book_author":id};

        let mut cursor = self
            .col
            .find(filter, None)
            .await
            .ok()
            .expect("Error fetch books by author");

        let mut books:Vec<Book> = Vec::new();

        while let Some(book) = cursor
            .try_next()
            .await
            .ok()
            .expect("Error mapping cursor")

        {
            books.push(book)
        }

        Ok(books)
    }

    pub async fn delete_books_by_author(&self, author_id:String) -> Result<DeleteResult, Error> {
        let filter = doc! {"book_author":author_id};

        let result = self
            .col
            .delete_many(filter, None)
            .await
            .ok()
            .expect("Error book delete");

        Ok(result)
    }

}