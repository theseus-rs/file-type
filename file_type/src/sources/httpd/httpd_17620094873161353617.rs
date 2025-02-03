use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17620094873161353617: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "tex tfm",
    extensions: &["tfm"],
    media_types: &["application/x-tex-tfm"],
    internal_signatures: &[],
    related_formats: &[],
};
