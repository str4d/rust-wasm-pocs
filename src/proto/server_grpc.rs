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


// interface

pub trait CompactTxStreamer {
    fn get_latest_block(&self, o: ::grpc::RequestOptions, p: super::server::ChainSpec) -> ::grpc::SingleResponse<super::block::BlockID>;

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::block::BlockID) -> ::grpc::SingleResponse<super::block::CompactBlock>;

    fn get_block_range(&self, o: ::grpc::RequestOptions, p: super::server::RangeFilter) -> ::grpc::StreamingResponse<super::block::CompactBlock>;
}

// client

pub struct CompactTxStreamerClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetLatestBlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::server::ChainSpec, super::block::BlockID>>,
    method_GetBlock: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::block::BlockID, super::block::CompactBlock>>,
    method_GetBlockRange: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::server::RangeFilter, super::block::CompactBlock>>,
}

impl ::grpc::ClientStub for CompactTxStreamerClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        CompactTxStreamerClient {
            grpc_client: grpc_client,
            method_GetLatestBlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CompactTxStreamer/GetLatestBlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetBlock: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CompactTxStreamer/GetBlock".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetBlockRange: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/proto.CompactTxStreamer/GetBlockRange".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl CompactTxStreamer for CompactTxStreamerClient {
    fn get_latest_block(&self, o: ::grpc::RequestOptions, p: super::server::ChainSpec) -> ::grpc::SingleResponse<super::block::BlockID> {
        self.grpc_client.call_unary(o, p, self.method_GetLatestBlock.clone())
    }

    fn get_block(&self, o: ::grpc::RequestOptions, p: super::block::BlockID) -> ::grpc::SingleResponse<super::block::CompactBlock> {
        self.grpc_client.call_unary(o, p, self.method_GetBlock.clone())
    }

    fn get_block_range(&self, o: ::grpc::RequestOptions, p: super::server::RangeFilter) -> ::grpc::StreamingResponse<super::block::CompactBlock> {
        self.grpc_client.call_server_streaming(o, p, self.method_GetBlockRange.clone())
    }
}

// server

pub struct CompactTxStreamerServer;


impl CompactTxStreamerServer {
    pub fn new_service_def<H : CompactTxStreamer + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/proto.CompactTxStreamer",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CompactTxStreamer/GetLatestBlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_latest_block(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CompactTxStreamer/GetBlock".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_block(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/proto.CompactTxStreamer/GetBlockRange".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::ServerStreaming,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerServerStreaming::new(move |o, p| handler_copy.get_block_range(o, p))
                    },
                ),
            ],
        )
    }
}
