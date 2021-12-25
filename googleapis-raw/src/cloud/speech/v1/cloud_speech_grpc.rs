// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_SPEECH_RECOGNIZE: ::grpcio::Method<super::cloud_speech::RecognizeRequest, super::cloud_speech::RecognizeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.speech.v1.Speech/Recognize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPEECH_LONG_RUNNING_RECOGNIZE: ::grpcio::Method<super::cloud_speech::LongRunningRecognizeRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.speech.v1.Speech/LongRunningRecognize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SPEECH_STREAMING_RECOGNIZE: ::grpcio::Method<super::cloud_speech::StreamingRecognizeRequest, super::cloud_speech::StreamingRecognizeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/google.cloud.speech.v1.Speech/StreamingRecognize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct SpeechClient {
    client: ::grpcio::Client,
}

impl SpeechClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SpeechClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn recognize_opt(&self, req: &super::cloud_speech::RecognizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::cloud_speech::RecognizeResponse> {
        self.client.unary_call(&METHOD_SPEECH_RECOGNIZE, req, opt)
    }

    pub fn recognize(&self, req: &super::cloud_speech::RecognizeRequest) -> ::grpcio::Result<super::cloud_speech::RecognizeResponse> {
        self.recognize_opt(req, ::grpcio::CallOption::default())
    }

    pub fn recognize_async_opt(&self, req: &super::cloud_speech::RecognizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cloud_speech::RecognizeResponse>> {
        self.client.unary_call_async(&METHOD_SPEECH_RECOGNIZE, req, opt)
    }

    pub fn recognize_async(&self, req: &super::cloud_speech::RecognizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::cloud_speech::RecognizeResponse>> {
        self.recognize_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn long_running_recognize_opt(&self, req: &super::cloud_speech::LongRunningRecognizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_SPEECH_LONG_RUNNING_RECOGNIZE, req, opt)
    }

    pub fn long_running_recognize(&self, req: &super::cloud_speech::LongRunningRecognizeRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.long_running_recognize_opt(req, ::grpcio::CallOption::default())
    }

    pub fn long_running_recognize_async_opt(&self, req: &super::cloud_speech::LongRunningRecognizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_SPEECH_LONG_RUNNING_RECOGNIZE, req, opt)
    }

    pub fn long_running_recognize_async(&self, req: &super::cloud_speech::LongRunningRecognizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.long_running_recognize_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn streaming_recognize_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::cloud_speech::StreamingRecognizeRequest>, ::grpcio::ClientDuplexReceiver<super::cloud_speech::StreamingRecognizeResponse>)> {
        self.client.duplex_streaming(&METHOD_SPEECH_STREAMING_RECOGNIZE, opt)
    }

    pub fn streaming_recognize(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::cloud_speech::StreamingRecognizeRequest>, ::grpcio::ClientDuplexReceiver<super::cloud_speech::StreamingRecognizeResponse>)> {
        self.streaming_recognize_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Speech {
    fn recognize(&mut self, ctx: ::grpcio::RpcContext, _req: super::cloud_speech::RecognizeRequest, sink: ::grpcio::UnarySink<super::cloud_speech::RecognizeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn long_running_recognize(&mut self, ctx: ::grpcio::RpcContext, _req: super::cloud_speech::LongRunningRecognizeRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn streaming_recognize(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::cloud_speech::StreamingRecognizeRequest>, sink: ::grpcio::DuplexSink<super::cloud_speech::StreamingRecognizeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_speech<S: Speech + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPEECH_RECOGNIZE, move |ctx, req, resp| {
        instance.recognize(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SPEECH_LONG_RUNNING_RECOGNIZE, move |ctx, req, resp| {
        instance.long_running_recognize(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_SPEECH_STREAMING_RECOGNIZE, move |ctx, req, resp| {
        instance.streaming_recognize(ctx, req, resp)
    });
    builder.build()
}
