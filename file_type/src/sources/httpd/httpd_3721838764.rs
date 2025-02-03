use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3721838764: FileFormat = FileFormat {
    id: 3_721_838_764,
    source_type: SourceType::Httpd,
    name: "shana informed package",
    extensions: &["ipk"],
    media_types: &["application/vnd.shana.informed.package"],
    internal_signatures: &[],
    related_formats: &[],
};
