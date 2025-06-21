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

use std::collections::HashMap;

/// Creates row
///
/// # Examples
///
/// ```
/// use jsdb::JSDB;
///
/// let mut project = JSDB::new();
///
/// if let Some(database) = project.databases.get_mut("database name") {
///     if let Some(table) = database.tables.get_mut("table name") {
///         if let Some(row) = table.rows.get_mut(&1) {
///             row.create_column("column name", String::from("hello"));
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Row<Value> {
    pub columns: HashMap<String, Value>,
}

impl<Value> Row<Value> {
    /// Creates a row that holds columns
    pub fn create_row() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    /// Adds a column to the existing row
    pub fn create_column(&mut self, name: &str, value: Value) {
        self.columns.insert(name.to_string(), value);
    }

    /// Removes a column from the existing row
    pub fn delete_column(&mut self, name: &str) {
        self.columns.remove(name);
    }
}
