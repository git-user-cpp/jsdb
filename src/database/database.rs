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

use super::table::Table;
use std::collections::HashMap;

/// Creates database
///
/// # Examples
///
/// ```
/// use jsdb::JSDB;
/// use jsdb::database::table::Table;
///
/// let mut project: JSDB<String> = JSDB::new();
/// if let Some(database) = project.databases.get_mut("database name") {
///     let tb1 = Table::create_table();
///     database.add_table("table name", tb1);
/// }
/// ```
#[derive(Debug)]
pub struct Database<Value> {
    pub tables: HashMap<String, Table<Value>>,
}

impl<Value> Database<Value> {
    /// Creates a database that holds tables
    pub fn create_database() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    /// Adds a table to the existing database
    pub fn add_table(&mut self, name: &str, table: Table<Value>) {
        self.tables.insert(name.to_string(), table);
    }

    /// Removes a table from the existing database
    pub fn delete_table(&mut self, name: &str) {
        self.tables.remove(name);
    }
}
