extern crate grpc_calculator_example;
extern crate grpcio;
extern crate futures;

use grpc_calculator_example::*;
use proto::Calculator;

use std::sync::Arc;
use futures::future::Future;
use grpcio::{ChannelBuilder, EnvBuilder, Environment,
             RpcContext, ServerBuilder, UnarySink};

#[derive(Clone)]
struct CalculatorService;

impl Calculator for CalculatorService {
   fn square_root(&self, ctx: RpcContext, req: proto::Number,
                  sink: UnarySink<proto::Number>) {
       let mut response = proto::Number::new();
       response.set_value((req.get_value()).sqrt());
       let f = sink.success(response)
           .map_err(move |e| println!("failed to reply {:?}: {:?}", req, e));
       ctx.spawn(f);
   }
}

fn main() {
    // let proto_dir = Path::new(env!("CARGO_MANIFEST_DIR")).
    //     		join("src/proto");

    // generate_proto(&proto_dir);

    let env = Arc::new(Environment::new(1));
    let service = proto::create_calculator(CalculatorService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 50051)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = proto::CalculatorClient::new(ch);

    let mut number = proto::Number::new();
    number.set_value(100.0);

    let reply = client.square_root(&number).expect("rpc");
    println!("Number received: {}", reply.get_value());
    let _ = server.shutdown().wait();
}
