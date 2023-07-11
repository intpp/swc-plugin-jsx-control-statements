use swc_core::common::errors::HANDLER;
use swc_core::common::Span;
use tracing::error;

pub fn display_error(span: Span, message: &str) {
    HANDLER.with(|handler| handler.struct_span_err(span, message).emit());

    error!(message);
}
