use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_482181358543897709: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "csv",
    extensions: &["csv"],
    media_types: &["text/csv"],
    internal_signatures: &[],
    related_formats: &[],
};
