use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_5659107803445623294: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "macports portpkg",
    extensions: &["portpkg"],
    media_types: &["application/vnd.macports.portpkg"],
    internal_signatures: &[],
    related_formats: &[],
};
