use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_16922485787373200833: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "java vm",
    extensions: &["class"],
    media_types: &["application/java-vm"],
    internal_signatures: &[],
    related_formats: &[],
};
