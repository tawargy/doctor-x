use super::db::get_db_connection;
use crate::types::Patient;
use rusqlite::Result;
use tauri::Manager;

pub fn search_patients_db(
    search_term: String,
    window: tauri::Window,
) -> Result<Vec<Patient>, String> {
    let conn = match get_db_connection(window.app_handle()) {
        Ok(conn) => conn,
        Err(_) => return Err(String::from("Failed to connect to database!")),
    };

    // Create a pattern that matches from the start
    let name_pattern = format!("{}%", search_term);

    let query = "
        SELECT * FROM patients
        WHERE lower(name) LIKE lower(?)
        OR phone LIKE ?
        ORDER BY
            CASE
                WHEN lower(name) LIKE lower(?) THEN 0
                WHEN phone LIKE ? THEN 1
                ELSE 2
            END,
            name
    ";

    println!("Search pattern: {}", name_pattern); // Debug log

    let mut stmt = conn.prepare(query).map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map(
            [&name_pattern, &name_pattern, &name_pattern, &name_pattern],
            |row| {
                Ok(Patient {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    dob: row.get(2)?,
                    age: row.get(3)?,
                    gender: row.get(4)?,
                    marital_status: row.get(5)?,
                    born_city: row.get(6)?,
                    residence: row.get(7)?,
                    occupation: row.get(8)?,
                    phone: row.get(9)?,
                    email: row.get(10)?,
                    insurance_provider: row.get(11)?,
                    insurance_policy_number: row.get(12)?,
                    insurance_group_number: row.get(13)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    let mut patients = Vec::new();
    for row in rows {
        match row {
            Ok(patient) => {
                println!("Found patient: {:?}", patient); // Debug log
                patients.push(patient);
            }
            Err(e) => println!("Error reading row: {}", e), // Debug log
        }
    }

    println!("Total patients found: {}", patients.len()); // Debug log
    Ok(patients)
}
