use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_8181556888019162819: FileFormat = FileFormat {
    id: 4_294_967_295,
    source_type: SourceType::Httpd,
    name: "aac",
    extensions: &["aac"],
    media_types: &["audio/x-aac"],
    internal_signatures: &[],
    related_formats: &[],
};
