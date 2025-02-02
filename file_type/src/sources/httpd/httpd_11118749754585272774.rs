use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_11118749754585272774: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "sun j2me app descriptor",
    extensions: &["jad"],
    media_types: &["text/vnd.sun.j2me.app-descriptor"],
    internal_signatures: &[],
    related_formats: &[],
};
