use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_2981768161253577861: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "java archive",
    extensions: &["jar"],
    media_types: &["application/java-archive"],
    internal_signatures: &[],
    related_formats: &[],
};
