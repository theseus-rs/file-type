use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_17692487408723072806: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "scvp cv response",
    extensions: &["scs"],
    media_types: &["application/scvp-cv-response"],
    internal_signatures: &[],
    related_formats: &[],
};
