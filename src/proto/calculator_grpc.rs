// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_CALCULATOR_SQUARE_ROOT: ::grpcio::Method<super::calculator::Number, super::calculator::Number> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/Calculator/SquareRoot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct CalculatorClient {
    client: ::grpcio::Client,
}

impl CalculatorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        CalculatorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn square_root_opt(&self, req: &super::calculator::Number, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::calculator::Number> {
        self.client.unary_call(&METHOD_CALCULATOR_SQUARE_ROOT, req, opt)
    }

    pub fn square_root(&self, req: &super::calculator::Number) -> ::grpcio::Result<super::calculator::Number> {
        self.square_root_opt(req, ::grpcio::CallOption::default())
    }

    pub fn square_root_async_opt(&self, req: &super::calculator::Number, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calculator::Number>> {
        self.client.unary_call_async(&METHOD_CALCULATOR_SQUARE_ROOT, req, opt)
    }

    pub fn square_root_async(&self, req: &super::calculator::Number) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::calculator::Number>> {
        self.square_root_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Calculator {
    fn square_root(&self, ctx: ::grpcio::RpcContext, req: super::calculator::Number, sink: ::grpcio::UnarySink<super::calculator::Number>);
}

pub fn create_calculator<S: Calculator + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_CALCULATOR_SQUARE_ROOT, move |ctx, req, resp| {
        instance.square_root(ctx, req, resp)
    });
    builder.build()
}
