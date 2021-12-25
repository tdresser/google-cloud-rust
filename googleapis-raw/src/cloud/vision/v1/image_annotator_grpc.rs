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

const METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_IMAGES: ::grpcio::Method<super::image_annotator::BatchAnnotateImagesRequest, super::image_annotator::BatchAnnotateImagesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ImageAnnotator/BatchAnnotateImages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_FILES: ::grpcio::Method<super::image_annotator::BatchAnnotateFilesRequest, super::image_annotator::BatchAnnotateFilesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ImageAnnotator/BatchAnnotateFiles",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_IMAGES: ::grpcio::Method<super::image_annotator::AsyncBatchAnnotateImagesRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ImageAnnotator/AsyncBatchAnnotateImages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_FILES: ::grpcio::Method<super::image_annotator::AsyncBatchAnnotateFilesRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ImageAnnotator/AsyncBatchAnnotateFiles",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ImageAnnotatorClient {
    client: ::grpcio::Client,
}

impl ImageAnnotatorClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ImageAnnotatorClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn batch_annotate_images_opt(&self, req: &super::image_annotator::BatchAnnotateImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::image_annotator::BatchAnnotateImagesResponse> {
        self.client.unary_call(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_IMAGES, req, opt)
    }

    pub fn batch_annotate_images(&self, req: &super::image_annotator::BatchAnnotateImagesRequest) -> ::grpcio::Result<super::image_annotator::BatchAnnotateImagesResponse> {
        self.batch_annotate_images_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_annotate_images_async_opt(&self, req: &super::image_annotator::BatchAnnotateImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::image_annotator::BatchAnnotateImagesResponse>> {
        self.client.unary_call_async(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_IMAGES, req, opt)
    }

    pub fn batch_annotate_images_async(&self, req: &super::image_annotator::BatchAnnotateImagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::image_annotator::BatchAnnotateImagesResponse>> {
        self.batch_annotate_images_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_annotate_files_opt(&self, req: &super::image_annotator::BatchAnnotateFilesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::image_annotator::BatchAnnotateFilesResponse> {
        self.client.unary_call(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_FILES, req, opt)
    }

    pub fn batch_annotate_files(&self, req: &super::image_annotator::BatchAnnotateFilesRequest) -> ::grpcio::Result<super::image_annotator::BatchAnnotateFilesResponse> {
        self.batch_annotate_files_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_annotate_files_async_opt(&self, req: &super::image_annotator::BatchAnnotateFilesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::image_annotator::BatchAnnotateFilesResponse>> {
        self.client.unary_call_async(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_FILES, req, opt)
    }

    pub fn batch_annotate_files_async(&self, req: &super::image_annotator::BatchAnnotateFilesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::image_annotator::BatchAnnotateFilesResponse>> {
        self.batch_annotate_files_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn async_batch_annotate_images_opt(&self, req: &super::image_annotator::AsyncBatchAnnotateImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_IMAGES, req, opt)
    }

    pub fn async_batch_annotate_images(&self, req: &super::image_annotator::AsyncBatchAnnotateImagesRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.async_batch_annotate_images_opt(req, ::grpcio::CallOption::default())
    }

    pub fn async_batch_annotate_images_async_opt(&self, req: &super::image_annotator::AsyncBatchAnnotateImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_IMAGES, req, opt)
    }

    pub fn async_batch_annotate_images_async(&self, req: &super::image_annotator::AsyncBatchAnnotateImagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.async_batch_annotate_images_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn async_batch_annotate_files_opt(&self, req: &super::image_annotator::AsyncBatchAnnotateFilesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_FILES, req, opt)
    }

    pub fn async_batch_annotate_files(&self, req: &super::image_annotator::AsyncBatchAnnotateFilesRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.async_batch_annotate_files_opt(req, ::grpcio::CallOption::default())
    }

    pub fn async_batch_annotate_files_async_opt(&self, req: &super::image_annotator::AsyncBatchAnnotateFilesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_FILES, req, opt)
    }

    pub fn async_batch_annotate_files_async(&self, req: &super::image_annotator::AsyncBatchAnnotateFilesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.async_batch_annotate_files_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ImageAnnotator {
    fn batch_annotate_images(&mut self, ctx: ::grpcio::RpcContext, _req: super::image_annotator::BatchAnnotateImagesRequest, sink: ::grpcio::UnarySink<super::image_annotator::BatchAnnotateImagesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_annotate_files(&mut self, ctx: ::grpcio::RpcContext, _req: super::image_annotator::BatchAnnotateFilesRequest, sink: ::grpcio::UnarySink<super::image_annotator::BatchAnnotateFilesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn async_batch_annotate_images(&mut self, ctx: ::grpcio::RpcContext, _req: super::image_annotator::AsyncBatchAnnotateImagesRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn async_batch_annotate_files(&mut self, ctx: ::grpcio::RpcContext, _req: super::image_annotator::AsyncBatchAnnotateFilesRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_image_annotator<S: ImageAnnotator + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_IMAGES, move |ctx, req, resp| {
        instance.batch_annotate_images(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMAGE_ANNOTATOR_BATCH_ANNOTATE_FILES, move |ctx, req, resp| {
        instance.batch_annotate_files(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_IMAGES, move |ctx, req, resp| {
        instance.async_batch_annotate_images(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_IMAGE_ANNOTATOR_ASYNC_BATCH_ANNOTATE_FILES, move |ctx, req, resp| {
        instance.async_batch_annotate_files(ctx, req, resp)
    });
    builder.build()
}
