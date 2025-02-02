use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_9575045232419729220: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "rls services xml",
    extensions: &["rs"],
    media_types: &["application/rls-services+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
