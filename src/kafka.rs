pub async fn receieve_messages(cfg: ServiceConfig){
    let consumer = create_consumer(..);

    let mut msg_stream = consumer.start();

    while let some(msg) = msg_stream.next().await{
        match mgs {
            Ok(msg)=> {

            }
            Err(e)=> error!("Could not receive and will not message: {}", e),
        };
    }

}


#[actix_web::main]
async fn main ()-> std::io ::Result<()>{

    actix_web::rt:: spawn(async move {receieve_messages(config).await});
}