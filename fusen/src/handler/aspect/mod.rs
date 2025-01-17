use std::sync::Arc;

use super::HandlerContext;
use crate::codec::request_codec::RequestCodec;
use crate::codec::response_codec::ResponseCodec;
use crate::register::ResourceInfo;
use crate::route::client::Route;
use crate::{
    codec::{request_codec::RequestHandler, response_codec::ResponseHandler},
    filter::FusenFilter,
    FusenFuture,
};
use fusen_common::error::FusenError;
use fusen_common::FusenContext;
use http_body_util::BodyExt;
use opentelemetry::propagation::TextMapPropagator;
use opentelemetry::trace::TraceContextExt;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[allow(async_fn_in_trait)]
pub trait Aspect {
    async fn aroud(
        &self,
        filter: &'static dyn FusenFilter,
        context: FusenContext,
    ) -> Result<FusenContext, crate::Error>;
}

pub trait Aspect_: Send + Sync {
    fn aroud_(
        &'static self,
        filter: &'static dyn FusenFilter,
        context: FusenContext,
    ) -> FusenFuture<Result<FusenContext, crate::Error>>;
}

pub struct DefaultAspect;

impl Aspect_ for DefaultAspect {
    fn aroud_(
        &'static self,
        filter: &'static dyn FusenFilter,
        context: FusenContext,
    ) -> FusenFuture<Result<FusenContext, crate::Error>> {
        Box::pin(async move { filter.call(context).await })
    }
}

pub struct AspectClientFilter {
    request_handle: RequestHandler,
    response_handle: ResponseHandler,
    handle_context: Arc<HandlerContext>,
    route: Route,
    trace_context_propagator: TraceContextPropagator,
}

impl AspectClientFilter {
    pub fn new(
        request_handle: RequestHandler,
        response_handle: ResponseHandler,
        handle_context: Arc<HandlerContext>,
        route: Route,
    ) -> Self {
        AspectClientFilter {
            request_handle,
            response_handle,
            handle_context,
            route,
            trace_context_propagator: TraceContextPropagator::new(),
        }
    }
}

impl FusenFilter for AspectClientFilter {
    fn call(
        &'static self,
        mut context: FusenContext,
    ) -> FusenFuture<Result<FusenContext, crate::Error>> {
        Box::pin(async move {
            let handler_controller = self
                .handle_context
                .get_controller(&context.get_context_info().get_handler_key());
            let resource_info: Arc<ResourceInfo> = self
                .route
                .get_server_resource(&context)
                .await
                .map_err(|e| FusenError::Info(e.to_string()))?;
            let socket = handler_controller
                .as_ref()
                .get_load_balance()
                .select_(resource_info)
                .await?;
            let span_context = Span::current().context();
            if span_context.has_active_span() {
                self.trace_context_propagator
                    .inject_context(&span_context, context.get_mut_request().get_mut_headers());
            }
            let request = self.request_handle.encode(&context)?;
            let response: http::Response<hyper::body::Incoming> =
                socket.send_request(request).await?;
            let res = self
                .response_handle
                .decode(response.map(|e| e.boxed()))
                .await;
            context.get_mut_response().set_response(res);
            Ok(context)
        })
    }
}
