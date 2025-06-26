/*
 * JSDB - Just a Simple DataBase.
 * Copyright (C) 2025  Andrew Kushyk
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

//! # Just a Simple DataBase
//!
//! `JSDB` is a database API designed for high-performance data management

pub mod database;
pub mod functionality;
pub mod queries;

use database::database::Database;
use std::collections::HashMap;

/// JSDB: A lightweight, high-performance database API for Rust.
///
/// The `JSDB` crate provides a simple, database management system designed for
/// ease of use and flexibility. It organizes data hierarchically with environments (`JSDB`),
/// databases (`Database`), tables (`Table`), rows (`Row`), and columns. This structure
/// supports generic data types, allowing developers to store various kinds of values.
///
/// # Features
/// - Create and manage multiple databases within a single environment.
/// - Support for generic value types in rows.
/// - Simple API for adding and removing databases.
///
/// # Usage
/// The `JSDB` struct is the entry point for interacting with the database system. Use it to
/// create an environment, add databases, and manage data.
///
/// # Examples
/// ```
/// use jsdb::JSDB;
/// use jsdb::database::database::Database;
///
/// // Create a new JSDB environment
/// let mut project: JSDB<String> = JSDB::new();
///
/// // Create a database and add it to the environment
/// let db1 = Database::create_database();
/// project.add_database("my_database", db1);
///
/// // Remove a database
/// project.delete_database("my_database");
/// ```
///
/// # Notes
/// - This is an in-memory database, so data is not persisted to disk.
/// - Ensure unique database names to avoid overwriting existing databases.
#[derive(Debug)]
pub struct JSDB<Value> {
    pub databases: HashMap<String, Database<Value>>,
}

impl<Value> JSDB<Value> {
    /// Creates an empty environment for holding databases
    pub fn new() -> Self {
        Self {
            databases: HashMap::new(),
        }
    }

    /// Adds database to the environment HashMap
    pub fn add_database(&mut self, name: &str, database: Database<Value>) {
        self.databases.insert(name.to_string(), database);
    }

    /// Removes database from the environment HashMap
    pub fn delete_database(&mut self, name: &str) {
        self.databases.remove(name);
    }
}
