use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2280517777109703612: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sgi movie",
    extensions: &["movie"],
    media_types: &["video/x-sgi-movie"],
    internal_signatures: &[],
    related_formats: &[],
};
