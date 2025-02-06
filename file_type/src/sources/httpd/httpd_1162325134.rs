use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_1162325134: FileFormat = FileFormat {
    id: 1_162_325_134,
    source_type: SourceType::Httpd,
    name: "java serialized object",
    extensions: &["ser"],
    media_types: &["application/java-serialized-object"],
    signatures: &[],
    related_formats: &[],
};
