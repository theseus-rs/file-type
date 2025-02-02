use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3641799221620537973: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "atom xml",
    extensions: &["atom"],
    media_types: &["application/atom+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
