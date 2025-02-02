use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13840748058500051469: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "wav",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    internal_signatures: &[],
    related_formats: &[],
};
