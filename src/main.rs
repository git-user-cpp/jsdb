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

use jsdb::JSDB;
use jsdb::database::database::Database;
use jsdb::database::row::Row;
use jsdb::database::table::Table;

fn main() {
    const DB_NAME: &str = "db1";
    const TB_NAME: &str = "tb1";
    const RW_NAME: u32 = 1;
    const CL_NAME: &str = "cl1";

    let mut project = JSDB::new();

    println!("{:#?}\n\n", project);

    let db1 = Database::create_database();
    project.add_database(DB_NAME, db1);

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        let tb1 = Table::create_table();
        database.add_table(TB_NAME, tb1);
    }

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        if let Some(table) = database.tables.get_mut(TB_NAME) {
            let rw1 = Row::create_row();
            table.add_row(RW_NAME, rw1);
        }
    }

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        if let Some(table) = database.tables.get_mut(TB_NAME) {
            if let Some(row) = table.rows.get_mut(&RW_NAME) {
                row.create_column(CL_NAME, String::from("hello"));
            }
        }
    }

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        if let Some(table) = database.tables.get_mut(TB_NAME) {
            if let Some(row) = table.rows.get_mut(&RW_NAME) {
                row.delete_column(CL_NAME);
            }
        }
    }

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        if let Some(table) = database.tables.get_mut(TB_NAME) {
            table.delete_row(RW_NAME);
        }
    }

    println!("{:#?}\n\n", project);

    if let Some(database) = project.databases.get_mut(DB_NAME) {
        database.delete_table(TB_NAME);
    }

    println!("{:#?}\n\n", project);

    project.delete_database(DB_NAME);

    println!("{:#?}", project);
}
