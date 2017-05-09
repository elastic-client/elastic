//! HTTP client, requests and responses.
//!
//! This module contains the HTTP client, as well
//! as request and response types.
//!
//! # The request process
//!
//! The pieces involved in sending a request and parsing the response are modular.
//! Each one exposes Rust traits you can implement to support your own logic.
//! If you just want to send search/get requests and parse a search/get response then
//! you won't need to worry about this so much.
//!
//! The basic flow from request to response is:
//!
//! **1)** Turn a concrete [request type](requests/endpoints/index.html) into a [`RequestBuilder`](struct.RequestBuilder.html):
//!
//! ```text
//! [RequestType] ---> [Client.request()] ---> [RequestBuilder]
//! ```
//!
//! **2)** Send the [`RequestBuilder`](struct.RequestBuilder.html) and get a [`ResponseBuilder`](struct.ResponseBuilder.html):
//!
//! ```text
//! [RequestBuilder.send()] ---> [ResponseBuilder]
//! ```
//!
//! **3)** Parse the [`ResponseBuilder`](struct.ResponseBuilder.html) to a [response type](responses/parse/trait.FromResponse.html#Implementors):
//!
//! ```text
//! [ResponseBuilder.response()] ---> [ResponseType]
//! ```
//!
//! The example below shows how these pieces fit together in code.
//!
//! # Examples
//!
//! This example sends a simple `SearchRequest`, with the steps in the above
//! process labelled:
//!
//! ```no_run
//! # extern crate elastic;
//! # #[macro_use]
//! # extern crate json_str;
//! # extern crate serde_json;
//! # use elastic::prelude::*;
//! # use elastic::error::*;
//! # use serde_json::Value;
//! # fn main() {
//! // Create a `Client`
//! let client = Client::new(RequestParams::default()).unwrap();
//!
//! // Create a `SearchRequest` for all indices
//! let req = {
//!     let body = json_str!({
//!         query: {
//!             query_string: {
//!                 query: "*"
//!             }
//!         }
//!     });
//!
//!     SearchRequest::for_index("_all", body)
//! };
//!
//! // Send the request and read the response as a `SearchResponse`
//! let res = client.request(req) // 1
//!                 .send() // 2
//!                 .and_then(|res| res.response::<SearchResponse<Value>>()); // 3
//!
//! match res {
//!     Ok(response) => {
//!         // Iterate through the response hits
//!         for hit in response.hits() {
//!             println!("{:?}", hit);
//!         }
//!     },
//!     Err(e) => {
//!         match *e.kind() {
//!             ErrorKind::Api(ref e) => {
//!                 // handle a REST API error
//!             },
//!             ref e => {
//!                 // handle a HTTP or JSON error
//!             }
//!         }
//!     }
//! }
//! # }
//! ```

pub mod requests;
pub mod responses;

use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use reqwest::{Client as HttpClient};

use error::*;
use self::requests::{IntoBody, HttpRequest, Index, Type};
use self::responses::HttpResponse;
use self::responses::parse::IsOk;

pub use elastic_reqwest::RequestParams;

/// A HTTP client for the Elasticsearch REST API.
///
/// The `Client` is a structure that lets you create and send `RequestBuilder`s.
/// It's mostly a thin wrapper over a `reqwest::Client`.
pub struct Client {
    http: HttpClient,
    params: RequestParams,
}

impl Client {
    /// Create a new client for the given parameters.
    ///
    /// The parameters given here are used as the defaults for any
    /// request made by this client, but can be overriden on a
    /// per-request basis.
    /// This method can return a `HttpError` if the underlying `reqwest::Client`
    /// fails to create.
    ///
    /// # Examples
    ///
    /// Create a `Client` with default parameters:
    ///
    /// ```
    /// # use elastic::prelude::*;
    /// let client = Client::new(RequestParams::default()).unwrap();
    /// ```
    ///
    /// Create a `Client` for a specific node:
    ///
    /// ```
    /// # use elastic::prelude::*;
    /// let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();
    /// ```
    ///
    /// See [`RequestParams`](struct.RequestParams.html) for more configuration options.
    pub fn new(params: RequestParams) -> Result<Self> {
        let client = HttpClient::new()?;

        Ok(Client {
            http: client,
            params: params,
        })
    }

    /// Create a `RequestBuilder` that can be configured before sending.
    ///
    /// The `request` method accepts any type that can be converted into
    /// a [`HttpRequest<'static>`](requests/struct.HttpRequest.html),
    /// which includes the endpoint types in the [`requests`](requests/endpoints/index.html) module.
    ///
    /// # Examples
    ///
    /// Turn a concrete request into a `RequestBuilder`:
    ///
    /// ```no_run
    /// # use elastic::prelude::*;
    /// # let client = Client::new(RequestParams::default()).unwrap();
    /// // `PingRequest` implements `Into<HttpRequest>`
    /// let req = PingRequest::new();
    ///
    /// // Turn the `PingRequest` into a `RequestBuilder`
    /// let builder = client.request(req);
    ///
    /// // Send the `RequestBuilder`
    /// let res = builder.send().unwrap();
    /// ```
    pub fn request<'a, TRequest, TBody>(&'a self,
                                        req: TRequest)
                                        -> RequestBuilder<'a, TRequest, TBody>
        where TRequest: Into<HttpRequest<'static, TBody>>,
              TBody: IntoBody
    {
        RequestBuilder::new(&self, None, req)
    }
}

/// Try convert a `ResponseBuilder` into a concrete response type.
pub fn into_response<T>(res: ResponseBuilder) -> Result<T>
    where T: IsOk + DeserializeOwned
{
    res.response()
}

/// Try convert a `ResponseBuilder` into a raw response type.
pub fn into_raw(res: ResponseBuilder) -> Result<HttpResponse> {
    Ok(res.raw())
}

/// A builder for a request.
///
/// This structure wraps up a concrete REST API request type
/// and lets you adjust parameters before sending it.
pub struct RequestBuilder<'a, TRequest, TBody> {
    client: &'a Client,
    params: Option<RequestParams>,
    req: TRequest,
    _marker: PhantomData<TBody>,
}

impl<'a, TRequest, TBody> RequestBuilder<'a, TRequest, TBody> {
    fn new(client: &'a Client, params: Option<RequestParams>, req: TRequest) -> Self {
        RequestBuilder {
            client: client,
            params: params,
            req: req,
            _marker: PhantomData,
        }
    }
}

/// A builder for a search request.
pub struct SearchRequestBuilder<TDocument, TBody> {
    index: Option<Index<'static>>,
    ty: Option<Type<'static>>,
    body: TBody,
    _marker: PhantomData<TDocument>,
}

/// A builder for a response.
///
/// This structure wraps the completed HTTP response but gives you
/// options for converting it into a concrete type.
/// You can also `Read` directly from the response body.
pub struct ResponseBuilder(HttpResponse);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_builder_params() {
        let client = Client::new(RequestParams::new("http://eshost:9200")).unwrap();

        let req = RequestBuilder::<_, _, ()>::new(&client, None, requests::PingRequest::new())
            .params(|p| p.url_param("pretty", true))
            .params(|p| p.url_param("refresh", true));

        let params = &req.params.unwrap();

        let (_, query) = params.get_url_qry();

        assert_eq!("http://eshost:9200", &params.base_url);
        assert_eq!("?pretty=true&refresh=true", query.unwrap());
    }
}
