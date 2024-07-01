

#[get("/vuln/{vulnerability_id}")]
pub async fn get_vulnerability(
    pool: web::Data<DbPool>,
    vuln_id:web::Path<i32>,
)-> Result<HttpResponse, Error>{

    let vuln_id = vuln_id.into_inner();

    let conn = pool.get().expect("couldnt get db connection");


    let vulnerability = web:: block(move || find_vulnerability_by_id(vuln_id, &conn))
        .await
        .map_err(|e|{
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;


        if let Some(vulnerability) = vulnerability{
            Ok(HttpResponse::Ok().json(vulnerability))
        }else {
            let res = HttpResponse::NotFound().body(format!("No vuln found with the id: {}", vul
            Ok(res)
        
        ));
        }
}



