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

const METHOD_PRODUCT_SEARCH_CREATE_PRODUCT_SET: ::grpcio::Method<super::product_search_service::CreateProductSetRequest, super::product_search_service::ProductSet> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/CreateProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_LIST_PRODUCT_SETS: ::grpcio::Method<super::product_search_service::ListProductSetsRequest, super::product_search_service::ListProductSetsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/ListProductSets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_GET_PRODUCT_SET: ::grpcio::Method<super::product_search_service::GetProductSetRequest, super::product_search_service::ProductSet> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/GetProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT_SET: ::grpcio::Method<super::product_search_service::UpdateProductSetRequest, super::product_search_service::ProductSet> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/UpdateProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_DELETE_PRODUCT_SET: ::grpcio::Method<super::product_search_service::DeleteProductSetRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/DeleteProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_CREATE_PRODUCT: ::grpcio::Method<super::product_search_service::CreateProductRequest, super::product_search_service::Product> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/CreateProduct",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_LIST_PRODUCTS: ::grpcio::Method<super::product_search_service::ListProductsRequest, super::product_search_service::ListProductsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/ListProducts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_GET_PRODUCT: ::grpcio::Method<super::product_search_service::GetProductRequest, super::product_search_service::Product> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/GetProduct",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT: ::grpcio::Method<super::product_search_service::UpdateProductRequest, super::product_search_service::Product> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/UpdateProduct",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_DELETE_PRODUCT: ::grpcio::Method<super::product_search_service::DeleteProductRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/DeleteProduct",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_CREATE_REFERENCE_IMAGE: ::grpcio::Method<super::product_search_service::CreateReferenceImageRequest, super::product_search_service::ReferenceImage> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/CreateReferenceImage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_DELETE_REFERENCE_IMAGE: ::grpcio::Method<super::product_search_service::DeleteReferenceImageRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/DeleteReferenceImage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_LIST_REFERENCE_IMAGES: ::grpcio::Method<super::product_search_service::ListReferenceImagesRequest, super::product_search_service::ListReferenceImagesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/ListReferenceImages",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_GET_REFERENCE_IMAGE: ::grpcio::Method<super::product_search_service::GetReferenceImageRequest, super::product_search_service::ReferenceImage> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/GetReferenceImage",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_ADD_PRODUCT_TO_PRODUCT_SET: ::grpcio::Method<super::product_search_service::AddProductToProductSetRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/AddProductToProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_REMOVE_PRODUCT_FROM_PRODUCT_SET: ::grpcio::Method<super::product_search_service::RemoveProductFromProductSetRequest, super::empty::Empty> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/RemoveProductFromProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_LIST_PRODUCTS_IN_PRODUCT_SET: ::grpcio::Method<super::product_search_service::ListProductsInProductSetRequest, super::product_search_service::ListProductsInProductSetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/ListProductsInProductSet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_IMPORT_PRODUCT_SETS: ::grpcio::Method<super::product_search_service::ImportProductSetsRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/ImportProductSets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_PRODUCT_SEARCH_PURGE_PRODUCTS: ::grpcio::Method<super::product_search_service::PurgeProductsRequest, super::operations::Operation> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/google.cloud.vision.v1.ProductSearch/PurgeProducts",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ProductSearchClient {
    client: ::grpcio::Client,
}

impl ProductSearchClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ProductSearchClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn create_product_set_opt(&self, req: &super::product_search_service::CreateProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT_SET, req, opt)
    }

    pub fn create_product_set(&self, req: &super::product_search_service::CreateProductSetRequest) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.create_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_product_set_async_opt(&self, req: &super::product_search_service::CreateProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT_SET, req, opt)
    }

    pub fn create_product_set_async(&self, req: &super::product_search_service::CreateProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.create_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_product_sets_opt(&self, req: &super::product_search_service::ListProductSetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ListProductSetsResponse> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_LIST_PRODUCT_SETS, req, opt)
    }

    pub fn list_product_sets(&self, req: &super::product_search_service::ListProductSetsRequest) -> ::grpcio::Result<super::product_search_service::ListProductSetsResponse> {
        self.list_product_sets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_product_sets_async_opt(&self, req: &super::product_search_service::ListProductSetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductSetsResponse>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_LIST_PRODUCT_SETS, req, opt)
    }

    pub fn list_product_sets_async(&self, req: &super::product_search_service::ListProductSetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductSetsResponse>> {
        self.list_product_sets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_product_set_opt(&self, req: &super::product_search_service::GetProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_GET_PRODUCT_SET, req, opt)
    }

    pub fn get_product_set(&self, req: &super::product_search_service::GetProductSetRequest) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.get_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_product_set_async_opt(&self, req: &super::product_search_service::GetProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_GET_PRODUCT_SET, req, opt)
    }

    pub fn get_product_set_async(&self, req: &super::product_search_service::GetProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.get_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_product_set_opt(&self, req: &super::product_search_service::UpdateProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT_SET, req, opt)
    }

    pub fn update_product_set(&self, req: &super::product_search_service::UpdateProductSetRequest) -> ::grpcio::Result<super::product_search_service::ProductSet> {
        self.update_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_product_set_async_opt(&self, req: &super::product_search_service::UpdateProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT_SET, req, opt)
    }

    pub fn update_product_set_async(&self, req: &super::product_search_service::UpdateProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ProductSet>> {
        self.update_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_product_set_opt(&self, req: &super::product_search_service::DeleteProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT_SET, req, opt)
    }

    pub fn delete_product_set(&self, req: &super::product_search_service::DeleteProductSetRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_product_set_async_opt(&self, req: &super::product_search_service::DeleteProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT_SET, req, opt)
    }

    pub fn delete_product_set_async(&self, req: &super::product_search_service::DeleteProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_product_opt(&self, req: &super::product_search_service::CreateProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::Product> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT, req, opt)
    }

    pub fn create_product(&self, req: &super::product_search_service::CreateProductRequest) -> ::grpcio::Result<super::product_search_service::Product> {
        self.create_product_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_product_async_opt(&self, req: &super::product_search_service::CreateProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT, req, opt)
    }

    pub fn create_product_async(&self, req: &super::product_search_service::CreateProductRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.create_product_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_products_opt(&self, req: &super::product_search_service::ListProductsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ListProductsResponse> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS, req, opt)
    }

    pub fn list_products(&self, req: &super::product_search_service::ListProductsRequest) -> ::grpcio::Result<super::product_search_service::ListProductsResponse> {
        self.list_products_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_products_async_opt(&self, req: &super::product_search_service::ListProductsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductsResponse>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS, req, opt)
    }

    pub fn list_products_async(&self, req: &super::product_search_service::ListProductsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductsResponse>> {
        self.list_products_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_product_opt(&self, req: &super::product_search_service::GetProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::Product> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_GET_PRODUCT, req, opt)
    }

    pub fn get_product(&self, req: &super::product_search_service::GetProductRequest) -> ::grpcio::Result<super::product_search_service::Product> {
        self.get_product_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_product_async_opt(&self, req: &super::product_search_service::GetProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_GET_PRODUCT, req, opt)
    }

    pub fn get_product_async(&self, req: &super::product_search_service::GetProductRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.get_product_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_product_opt(&self, req: &super::product_search_service::UpdateProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::Product> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT, req, opt)
    }

    pub fn update_product(&self, req: &super::product_search_service::UpdateProductRequest) -> ::grpcio::Result<super::product_search_service::Product> {
        self.update_product_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_product_async_opt(&self, req: &super::product_search_service::UpdateProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT, req, opt)
    }

    pub fn update_product_async(&self, req: &super::product_search_service::UpdateProductRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::Product>> {
        self.update_product_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_product_opt(&self, req: &super::product_search_service::DeleteProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT, req, opt)
    }

    pub fn delete_product(&self, req: &super::product_search_service::DeleteProductRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_product_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_product_async_opt(&self, req: &super::product_search_service::DeleteProductRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT, req, opt)
    }

    pub fn delete_product_async(&self, req: &super::product_search_service::DeleteProductRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_product_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_reference_image_opt(&self, req: &super::product_search_service::CreateReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ReferenceImage> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_CREATE_REFERENCE_IMAGE, req, opt)
    }

    pub fn create_reference_image(&self, req: &super::product_search_service::CreateReferenceImageRequest) -> ::grpcio::Result<super::product_search_service::ReferenceImage> {
        self.create_reference_image_opt(req, ::grpcio::CallOption::default())
    }

    pub fn create_reference_image_async_opt(&self, req: &super::product_search_service::CreateReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ReferenceImage>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_CREATE_REFERENCE_IMAGE, req, opt)
    }

    pub fn create_reference_image_async(&self, req: &super::product_search_service::CreateReferenceImageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ReferenceImage>> {
        self.create_reference_image_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_reference_image_opt(&self, req: &super::product_search_service::DeleteReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_DELETE_REFERENCE_IMAGE, req, opt)
    }

    pub fn delete_reference_image(&self, req: &super::product_search_service::DeleteReferenceImageRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.delete_reference_image_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_reference_image_async_opt(&self, req: &super::product_search_service::DeleteReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_DELETE_REFERENCE_IMAGE, req, opt)
    }

    pub fn delete_reference_image_async(&self, req: &super::product_search_service::DeleteReferenceImageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.delete_reference_image_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_reference_images_opt(&self, req: &super::product_search_service::ListReferenceImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ListReferenceImagesResponse> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_LIST_REFERENCE_IMAGES, req, opt)
    }

    pub fn list_reference_images(&self, req: &super::product_search_service::ListReferenceImagesRequest) -> ::grpcio::Result<super::product_search_service::ListReferenceImagesResponse> {
        self.list_reference_images_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_reference_images_async_opt(&self, req: &super::product_search_service::ListReferenceImagesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListReferenceImagesResponse>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_LIST_REFERENCE_IMAGES, req, opt)
    }

    pub fn list_reference_images_async(&self, req: &super::product_search_service::ListReferenceImagesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListReferenceImagesResponse>> {
        self.list_reference_images_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_reference_image_opt(&self, req: &super::product_search_service::GetReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ReferenceImage> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_GET_REFERENCE_IMAGE, req, opt)
    }

    pub fn get_reference_image(&self, req: &super::product_search_service::GetReferenceImageRequest) -> ::grpcio::Result<super::product_search_service::ReferenceImage> {
        self.get_reference_image_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_reference_image_async_opt(&self, req: &super::product_search_service::GetReferenceImageRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ReferenceImage>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_GET_REFERENCE_IMAGE, req, opt)
    }

    pub fn get_reference_image_async(&self, req: &super::product_search_service::GetReferenceImageRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ReferenceImage>> {
        self.get_reference_image_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_product_to_product_set_opt(&self, req: &super::product_search_service::AddProductToProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_ADD_PRODUCT_TO_PRODUCT_SET, req, opt)
    }

    pub fn add_product_to_product_set(&self, req: &super::product_search_service::AddProductToProductSetRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.add_product_to_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_product_to_product_set_async_opt(&self, req: &super::product_search_service::AddProductToProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_ADD_PRODUCT_TO_PRODUCT_SET, req, opt)
    }

    pub fn add_product_to_product_set_async(&self, req: &super::product_search_service::AddProductToProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.add_product_to_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_product_from_product_set_opt(&self, req: &super::product_search_service::RemoveProductFromProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::empty::Empty> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_REMOVE_PRODUCT_FROM_PRODUCT_SET, req, opt)
    }

    pub fn remove_product_from_product_set(&self, req: &super::product_search_service::RemoveProductFromProductSetRequest) -> ::grpcio::Result<super::empty::Empty> {
        self.remove_product_from_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_product_from_product_set_async_opt(&self, req: &super::product_search_service::RemoveProductFromProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_REMOVE_PRODUCT_FROM_PRODUCT_SET, req, opt)
    }

    pub fn remove_product_from_product_set_async(&self, req: &super::product_search_service::RemoveProductFromProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::empty::Empty>> {
        self.remove_product_from_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_products_in_product_set_opt(&self, req: &super::product_search_service::ListProductsInProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::product_search_service::ListProductsInProductSetResponse> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS_IN_PRODUCT_SET, req, opt)
    }

    pub fn list_products_in_product_set(&self, req: &super::product_search_service::ListProductsInProductSetRequest) -> ::grpcio::Result<super::product_search_service::ListProductsInProductSetResponse> {
        self.list_products_in_product_set_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_products_in_product_set_async_opt(&self, req: &super::product_search_service::ListProductsInProductSetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductsInProductSetResponse>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS_IN_PRODUCT_SET, req, opt)
    }

    pub fn list_products_in_product_set_async(&self, req: &super::product_search_service::ListProductsInProductSetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::product_search_service::ListProductsInProductSetResponse>> {
        self.list_products_in_product_set_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_product_sets_opt(&self, req: &super::product_search_service::ImportProductSetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_IMPORT_PRODUCT_SETS, req, opt)
    }

    pub fn import_product_sets(&self, req: &super::product_search_service::ImportProductSetsRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.import_product_sets_opt(req, ::grpcio::CallOption::default())
    }

    pub fn import_product_sets_async_opt(&self, req: &super::product_search_service::ImportProductSetsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_IMPORT_PRODUCT_SETS, req, opt)
    }

    pub fn import_product_sets_async(&self, req: &super::product_search_service::ImportProductSetsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.import_product_sets_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn purge_products_opt(&self, req: &super::product_search_service::PurgeProductsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::operations::Operation> {
        self.client.unary_call(&METHOD_PRODUCT_SEARCH_PURGE_PRODUCTS, req, opt)
    }

    pub fn purge_products(&self, req: &super::product_search_service::PurgeProductsRequest) -> ::grpcio::Result<super::operations::Operation> {
        self.purge_products_opt(req, ::grpcio::CallOption::default())
    }

    pub fn purge_products_async_opt(&self, req: &super::product_search_service::PurgeProductsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.client.unary_call_async(&METHOD_PRODUCT_SEARCH_PURGE_PRODUCTS, req, opt)
    }

    pub fn purge_products_async(&self, req: &super::product_search_service::PurgeProductsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::operations::Operation>> {
        self.purge_products_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ProductSearch {
    fn create_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::CreateProductSetRequest, sink: ::grpcio::UnarySink<super::product_search_service::ProductSet>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_product_sets(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::ListProductSetsRequest, sink: ::grpcio::UnarySink<super::product_search_service::ListProductSetsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::GetProductSetRequest, sink: ::grpcio::UnarySink<super::product_search_service::ProductSet>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::UpdateProductSetRequest, sink: ::grpcio::UnarySink<super::product_search_service::ProductSet>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::DeleteProductSetRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_product(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::CreateProductRequest, sink: ::grpcio::UnarySink<super::product_search_service::Product>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_products(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::ListProductsRequest, sink: ::grpcio::UnarySink<super::product_search_service::ListProductsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_product(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::GetProductRequest, sink: ::grpcio::UnarySink<super::product_search_service::Product>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn update_product(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::UpdateProductRequest, sink: ::grpcio::UnarySink<super::product_search_service::Product>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_product(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::DeleteProductRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn create_reference_image(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::CreateReferenceImageRequest, sink: ::grpcio::UnarySink<super::product_search_service::ReferenceImage>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_reference_image(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::DeleteReferenceImageRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_reference_images(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::ListReferenceImagesRequest, sink: ::grpcio::UnarySink<super::product_search_service::ListReferenceImagesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_reference_image(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::GetReferenceImageRequest, sink: ::grpcio::UnarySink<super::product_search_service::ReferenceImage>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn add_product_to_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::AddProductToProductSetRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn remove_product_from_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::RemoveProductFromProductSetRequest, sink: ::grpcio::UnarySink<super::empty::Empty>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_products_in_product_set(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::ListProductsInProductSetRequest, sink: ::grpcio::UnarySink<super::product_search_service::ListProductsInProductSetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn import_product_sets(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::ImportProductSetsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn purge_products(&mut self, ctx: ::grpcio::RpcContext, _req: super::product_search_service::PurgeProductsRequest, sink: ::grpcio::UnarySink<super::operations::Operation>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_product_search<S: ProductSearch + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT_SET, move |ctx, req, resp| {
        instance.create_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_LIST_PRODUCT_SETS, move |ctx, req, resp| {
        instance.list_product_sets(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_GET_PRODUCT_SET, move |ctx, req, resp| {
        instance.get_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT_SET, move |ctx, req, resp| {
        instance.update_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT_SET, move |ctx, req, resp| {
        instance.delete_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_CREATE_PRODUCT, move |ctx, req, resp| {
        instance.create_product(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS, move |ctx, req, resp| {
        instance.list_products(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_GET_PRODUCT, move |ctx, req, resp| {
        instance.get_product(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_UPDATE_PRODUCT, move |ctx, req, resp| {
        instance.update_product(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_DELETE_PRODUCT, move |ctx, req, resp| {
        instance.delete_product(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_CREATE_REFERENCE_IMAGE, move |ctx, req, resp| {
        instance.create_reference_image(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_DELETE_REFERENCE_IMAGE, move |ctx, req, resp| {
        instance.delete_reference_image(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_LIST_REFERENCE_IMAGES, move |ctx, req, resp| {
        instance.list_reference_images(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_GET_REFERENCE_IMAGE, move |ctx, req, resp| {
        instance.get_reference_image(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_ADD_PRODUCT_TO_PRODUCT_SET, move |ctx, req, resp| {
        instance.add_product_to_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_REMOVE_PRODUCT_FROM_PRODUCT_SET, move |ctx, req, resp| {
        instance.remove_product_from_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_LIST_PRODUCTS_IN_PRODUCT_SET, move |ctx, req, resp| {
        instance.list_products_in_product_set(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_IMPORT_PRODUCT_SETS, move |ctx, req, resp| {
        instance.import_product_sets(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_PRODUCT_SEARCH_PURGE_PRODUCTS, move |ctx, req, resp| {
        instance.purge_products(ctx, req, resp)
    });
    builder.build()
}
