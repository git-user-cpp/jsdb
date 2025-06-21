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

/// Manages tables within a database in the JSDB system.
///
/// This module defines the `Database` struct, which serves as a container for tables
/// in the JSDB hierarchy. Each database holds a collection of tables, identified by
/// unique names, and supports generic data types for flexible storage. It provides
/// methods to create, add, and remove tables, integrating with the broader JSDB
/// environment.
///
/// # Role in JSDB
/// The `Database` struct is a mid-level component in the JSDB hierarchy, sitting
/// between the `JSDB` environment and individual `Table` instances. It enables
/// organization of data into tables, which in turn contain rows and columns.
use super::table::Table;
use std::collections::HashMap;

/// A database that organizes tables in the JSDB system.
///
/// The `Database` struct stores a collection of tables, each identified by a unique
/// name, in a `HashMap`. It provides methods to manage tables, such as creating a new
/// database, adding tables, and removing them. This struct is generic, allowing
/// flexible storage of various data types in its tables.
///
/// # Type Parameters
/// - `Value`: The type of data stored in the rows of the tables within this database.
///
/// # Examples
/// ```
/// use jsdb::JSDB;
/// use jsdb::database::database::Database;
/// use jsdb::database::table::Table;
///
/// // Create a JSDB environment and add a database
/// let mut project: JSDB<String> = JSDB::new();
/// let db = Database::create_database();
/// project.add_database("my_db", db);
///
/// // Add a table to the database
/// if let Some(database) = project.databases.get_mut("my_db") {
///     let table = Table::create_table();
///     database.add_table("my_table", table);
/// }
/// ```
///
/// # Notes
/// - Databases are stored in-memory and do not persist data to disk.
/// - Table names within a database must be unique to avoid overwriting.
#[derive(Debug)]
pub struct Database<Value> {
    /// A collection of tables, mapped by their names.
    pub tables: HashMap<String, Table<Value>>,
}

impl<Value> Database<Value> {
    /// Creates a new, empty database.
    ///
    /// Initializes a `Database` instance with an empty `HashMap` to store tables.
    ///
    /// # Returns
    /// A new `Database` instance.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::database::Database;
    /// let db: Database<String> = Database::create_database();
    /// assert!(db.tables.is_empty());
    /// ```
    pub fn create_database() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    /// Adds a table to the database.
    ///
    /// Inserts a `Table` into the database under the specified name. If a table with
    /// the same name already exists, it is replaced.
    ///
    /// # Arguments
    /// - `name`: The unique name for the table.
    /// - `table`: The `Table` instance to add.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::database::Database;
    /// use jsdb::database::table::Table;
    /// let mut db: Database<String> = Database::create_database();
    /// let table = Table::create_table();
    /// db.add_table("my_table", table);
    /// assert!(db.tables.contains_key("my_table"));
    /// ```
    pub fn add_table(&mut self, name: &str, table: Table<Value>) {
        self.tables.insert(name.to_string(), table);
    }

    /// Removes a table from the database.
    ///
    /// Deletes the table with the specified name. If no table exists with that name,
    /// the operation has no effect.
    ///
    /// # Arguments
    /// - `name`: The name of the table to remove.
    ///
    /// # Examples
    /// ```
    /// use jsdb::database::database::Database;
    /// use jsdb::database::table::Table;
    /// let mut db: Database<String> = Database::create_database();
    /// let table = Table::create_table();
    /// db.add_table("my_table", table);
    /// db.delete_table("my_table");
    /// assert!(db.tables.is_empty());
    /// ```
    pub fn delete_table(&mut self, name: &str) {
        self.tables.remove(name);
    }
}
