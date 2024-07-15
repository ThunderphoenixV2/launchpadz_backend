#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

pub mod schema;
pub mod models;
pub mod db;
pub use db::{*, handle_users::handle_users::*};

// ===================================================================== //

// https://www.tutorialsteacher.com/postgresql/create-tables


// create table:
//
// CREATE TABLE [IF NOT EXISTS] <table_name> (
//     <column1> <data_type(length)> [column_contraint],
//     <column2> <data_type(length)> [column_contraint],
// ...
//     <columnN> <data_type(length)> [column_contraint],
//     [table_constraints]
// );


// add values to a table:
//
// INSERT INTO <table-name> (<column1>, <column2>,...)
// VALUES (<value1>, <value2>,...)
// RETURNING * or <column_name>;


// change values in a table:
//
// UPDATE <table_name>
// SET <column1> = <value1>,
//     <column2> = <value2>,
//     ...
// WHERE <condition>
// RETURNING * | <output_expression> AS <output_name>;


// delete data:
//
// DELETE FROM <table_name>
// [WHERE <condition<]
// RETURNING * | <output_expression< AS <output_name<;

// ===================================================================== //

// error functions
#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
