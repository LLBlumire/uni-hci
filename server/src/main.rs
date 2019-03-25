#![feature(decl_macro, proc_macro_hygiene)]

use std::ops::Deref;
use std::collections::HashMap;

use rocket::routes;
use rocket::get;
use rocket::http::Status;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket_contrib::database;
use rocket_contrib::databases::mysql;
use serde::Serialize;

const API_FOODS_QUERY: &str = r#"
    SELECT
        FOOD_DES.Long_Desc,
        FOOD_DES.NDB_No
    FROM
        FOOD_DES
"#;

const API_FOOD_INFO: &str = r#"
    SELECT
        FOOD_DES.Long_Desc,
        FOOD_DES.Shrt_Desc,
        FD_GROUP.FdGrp_Desc
    FROM FOOD_DES
    JOIN FD_GROUP
        ON FOOD_DES.FdGrp_Cd = FD_GROUP.FdGrp_Cd
    WHERE FOOD_DES.NDB_No = ?;
"#;

const API_FOOD_NUT_INFO: &str = r#"
    SELECT
        NUT_DATA.Nutr_No,
        NUT_DATA.Nutr_Val
    FROM FOOD_DES
    JOIN NUT_DATA
        ON NUT_DATA.NDB_No = FOOD_DES.NDB_No
    JOIN NUTR_DEF
        ON NUTR_DEF.Nutr_No = NUT_DATA.Nutr_No
    WHERE FOOD_DES.NDB_No = ?;
"#;

#[database("hcidata")]
struct HciDataConn(mysql::Conn);

const PROTEIN_G: i64 = 203;
const FAT_G: i64 = 204;
const CARBS_G: i64 = 205;
const ENERGY_KCAL: i64 = 208;
const WATER_G: i64 = 255;
const SUGARS_G: i64 = 269;

#[get("/foods")]
fn api_foods(mut conn: HciDataConn) -> Json<HashMap<String, i64>> {
    println!("Getting Foods");
    Json(conn.0.query(API_FOODS_QUERY)
        .unwrap()
        .map(|row| {
            mysql::from_row(row.unwrap())
        }).collect())
}

#[derive(Serialize)]
struct FoodInfo {
    long_desc: String,
    shrt_desc: String,
    fd_grp_desc: String,
    protein_g: f64,
    fat_g: f64,
    carbs_g: f64,
    energy_kcal: f64,
    water_g: f64,
    sugars_g: f64,
}

#[get("/food/<id>")]
fn api_food_by_id(mut conn: HciDataConn, id: i64) -> Result<Json<FoodInfo>, Status> {
    println!("Getting food: {}", id);
    let long_desc: String;
    let shrt_desc: String;
    let fd_grp_desc: String;
    let mut protein_g: f64 = 0.0;
    let mut fat_g: f64 = 0.0;
    let mut carbs_g: f64 = 0.0;
    let mut energy_kcal: f64 = 0.0;
    let mut water_g: f64 = 0.0;
    let mut sugars_g: f64 = 0.0;
    {
        let mut food_q = conn.0.prepare(API_FOOD_INFO).expect("Invalid SQL");
        if let Ok(Some(food)) = food_q.first_exec((id,)) {
            let (l, s, g) = mysql::from_row(food);
            long_desc = l;
            shrt_desc = s;
            fd_grp_desc = g;
        } else {
            eprintln!("FAIL TO REOLVE FOOD ROWS");
            return Err(Status::NotFound);
        }
    }
    let mut nut_q = conn.0.prepare(API_FOOD_NUT_INFO).expect("Invalid SQL");
    if let Ok(nuts) = nut_q.execute((id,)) {
        for nuts in nuts.map(|row| row.map(|row| mysql::from_row::<(i64, f64)>(row))) {
            if let Ok((n, v)) = nuts {
                if n == PROTEIN_G { protein_g = v;  }
                if n == FAT_G { fat_g = v; }
                if n == CARBS_G { carbs_g = v; }
                if n == ENERGY_KCAL { energy_kcal = v; }
                if n == WATER_G { water_g = v; }
                if n == SUGARS_G { sugars_g = v; }
            } else {
                eprintln!("FAIL TO RESOLVE NUTRIENT ROWS");
                return Err(Status::NotFound);
            }
        }
    } else {
        eprintln!("FAIL TO EXECUTE NUTRIENT SQL");
        return Err(Status::NotFound)
    }

    Ok(Json(FoodInfo {
        long_desc, shrt_desc, fd_grp_desc, protein_g, fat_g, carbs_g, 
        energy_kcal, water_g, sugars_g
    }))
}

fn main() {
    rocket::ignite()
        .attach(HciDataConn::fairing())
        .mount("/api", routes![api_foods, api_food_by_id])
        .launch();
}
