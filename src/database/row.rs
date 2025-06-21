/*
 * JSDB - Just Simple DataBase.
 * Copyright (C) 2024-2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

/// Manages columns within a row in the JSDB system.
///
/// This module defines the `Row` struct, which represents a single row in a JSDB table.
/// A row stores data as a collection of columns, each identified by a unique name and
/// holding a value of a generic type. The `Row` struct provides methods to create rows,
/// add columns, and remove columns, serving as the fundamental data storage unit in JSDB.
///
/// # Role in JSDB
/// The `Row` struct is the lowest level in the JSDB data hierarchy, contained within
/// a `Table`. It enables flexible storage of key-value pairs, where keys are column
/// names and values are user-defined data types.
use std::collections::HashMap;

/// A row that stores columns as key-value pairs in a JSDB table.
///
/// The `Row` struct holds a collection of columns in a `HashMap`, where each column
/// is identified by a unique name and stores a value of the generic type `Value`.
/// It provides methods to create a row, add columns, and remove columns, allowing
/// fine-grained data management within a table.
///
/// # Type Parameters
/// - `Value`: The type of data stored in the columns of this row.
///
/// # Examples
/// ```
/// use jsdb::JSDB;
/// use jsdb::database::row::Row;
/// use jsdb::database::table::Table;
///
/// // Create a JSDB environment, database, and table
/// let mut project: JSDB<String> = JSDB::new();
/// let db = jsdb::database::database::Database::create_database();
/// project.add_database("my_db", db);
///
/// if let Some(database) = project.databases.get_mut("my_db") {
///     let table = Table::create_table();
///     database.add_table("my_table", table);
///
///     // Add a row with a column
///     if let Some(table) = database.tables.get_mut("my_table") {
///         let row = Row::create_row();
///         table.add_row(1, row);
///
///         if let Some(row) = table.rows.get_mut(&1) {
///             row.create_column("name", String::from("hello"));
///         }
///     }
/// }
/// ```
///
/// # Notes
/// - Rows are stored in-memory and do not persist data to disk.
/// - Column names within a row must be unique to avoid overwriting.
#[derive(Debug)]
pub struct Row<Value> {
    /// A collection of columns, mapped by their names.
    pub columns: HashMap<String, Value>,
}

impl<Value> Row<Value> {
    /// Creates a new, empty row.
    ///
    /// Initializes a `Row` instance with an empty `HashMap` to store columns.
    ///
    /// # Returns
    /// A new `Row` instance.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::row::Row;
    /// let row: Row<String> = Row::create_row();
    /// assert!(row.columns.is_empty());
    /// ```
    pub fn create_row() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    /// Adds a column to the row.
    ///
    /// Inserts a column with the specified name and value into the row’s `HashMap`.
    /// If a column with the same name already exists, it is replaced.
    ///
    /// # Arguments
    /// - `name`: The unique name for the column.
    /// - `value`: The value to store in the column.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::row::Row;
    /// let mut row: Row<String> = Row::create_row();
    /// row.create_column("name", String::from("hello"));
    /// assert_eq!(row.columns.get("name"), Some(&String::from("hello")));
    /// ```
    pub fn create_column(&mut self, name: &str, value: Value) {
        self.columns.insert(name.to_string(), value);
    }

    /// Removes a column from the row.
    ///
    /// Deletes the column with the specified name. If no column exists with that
    /// name, the operation has no effect.
    ///
    /// # Arguments
    /// - `name`: The name of the column to remove.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::row::Row;
    /// let mut row: Row<String> = Row::create_row();
    /// row.create_column("name", String::from("hello"));
    /// row.delete_column("name");
    /// assert!(row.columns.is_empty());
    /// ```
    pub fn delete_column(&mut self, name: &str) {
        self.columns.remove(name);
    }
}
