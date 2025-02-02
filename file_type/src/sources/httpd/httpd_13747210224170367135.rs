use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_13747210224170367135: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "fmi flexstor",
    extensions: &["flx"],
    media_types: &["text/vnd.fmi.flexstor"],
    internal_signatures: &[],
    related_formats: &[],
};
