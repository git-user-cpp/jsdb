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

/// Manages rows within a table in the JSDB system.
///
/// This module defines the `Table` struct, which represents a table in a JSDB database.
/// A table organizes data into rows, each identified by a unique `u32` primary key.
/// The `Table` struct provides methods to create tables, add rows, and remove rows,
/// serving as a critical component in the JSDB data hierarchy.
///
/// # Role in JSDB
/// The `Table` struct is a mid-level component in the JSDB hierarchy, contained within
/// a `Database` and holding `Row` instances. It enables structured storage of rows,
/// each of which contains columns with user-defined data types.
use super::row::Row;
use std::collections::HashMap;

/// A table that organizes rows in a JSDB database.
///
/// The `Table` struct stores a collection of rows in a `HashMap`, where each row is
/// identified by a unique `u32` primary key. It provides methods to create a table,
/// add rows, and remove rows, facilitating data organization within a database.
/// The struct is generic, allowing flexible storage of various data types in its rows.
///
/// # Type Parameters
/// - `Value`: The type of data stored in the columns of the rows within this table.
///
/// # Examples
/// ```
/// use jsdb::JSDB;
/// use jsdb::database::row::Row;
/// use jsdb::database::table::Table;
///
/// // Create a JSDB environment and database
/// let mut project: JSDB<String> = JSDB::new();
/// let db = jsdb::database::database::Database::create_database();
/// project.add_database("my_db", db);
///
/// // Add a table and a row
/// if let Some(database) = project.databases.get_mut("my_db") {
///     let table = Table::create_table();
///     database.add_table("my_table", table);
///
///     if let Some(table) = database.tables.get_mut("my_table") {
///         let row = Row::create_row();
///         table.add_row(1, row);
///     }
/// }
/// ```
///
/// # Notes
/// - Tables are stored in-memory and do not persist data to disk.
/// - Row primary keys must be unique within a table to avoid overwriting.
#[derive(Debug)]
pub struct Table<Value> {
    /// A collection of rows, mapped by their `u32` primary keys.
    pub rows: HashMap<u32, Row<Value>>,
}

impl<Value> Table<Value> {
    /// Creates a new, empty table.
    ///
    /// Initializes a `Table` instance with an empty `HashMap` to store rows.
    ///
    /// # Returns
    /// A new `Table` instance.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::table::Table;
    /// let table: Table<String> = Table::create_table();
    /// assert!(table.rows.is_empty());
    /// ```
    pub fn create_table() -> Self {
        Self {
            rows: HashMap::new(),
        }
    }

    /// Adds a row to the table.
    ///
    /// Inserts a `Row` into the table’s `HashMap` with the specified `u32` primary key.
    /// If a row with the same primary key already exists, it is replaced.
    ///
    /// # Arguments
    /// - `primary_key`: The unique `u32` identifier for the row.
    /// - `row`: The `Row` instance to add.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::table::Table;
    /// use jsdb::database::row::Row;
    /// let mut table: Table<String> = Table::create_table();
    /// let row = Row::create_row();
    /// table.add_row(1, row);
    /// assert!(table.rows.contains_key(&1));
    /// ```
    pub fn add_row(&mut self, primary_key: u32, row: Row<Value>) {
        self.rows.insert(primary_key, row);
    }

    /// Removes a row from the table.
    ///
    /// Deletes the row with the specified `u32` primary key. If no row exists with
    /// that primary key, the operation has no effect.
    ///
    /// # Arguments
    /// - `primary_key`: The `u32` identifier of the row to remove.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::table::Table;
    /// use jsdb::database::row::Row;
    /// let mut table: Table<String> = Table::create_table();
    /// let row = Row::create_row();
    /// table.add_row(1, row);
    /// table.delete_row(1);
    /// assert!(table.rows.is_empty());
    /// ```
    pub fn delete_row(&mut self, primary_key: u32) {
        self.rows.remove(&primary_key);
    }
}
