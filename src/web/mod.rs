//! Drop in types for actix web implementing garde
mod either;
mod form;
mod header;
mod json;
mod path;
#[cfg(feature = "serde_qs")]
mod qs_query;
#[cfg(feature = "lab_query")]
mod lab_query;
mod query;

pub use either::Either;
pub use form::{Form, FormConfig};
pub use header::Header;
pub use json::{Json, JsonConfig};
pub use path::{Path, PathConfig};
#[cfg(feature = "serde_qs")]
pub use qs_query::{QsQuery, QsQueryConfig};
#[cfg(feature = "lab_query")]
pub use lab_query::Query as LabQuery;
pub use query::{Query, QueryConfig};
