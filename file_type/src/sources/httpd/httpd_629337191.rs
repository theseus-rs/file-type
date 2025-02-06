use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_629337191: FileFormat = FileFormat {
    id: 629_337_191,
    source_type: SourceType::Httpd,
    name: "aac",
    extensions: &["aac"],
    media_types: &["audio/x-aac"],
    signatures: &[],
    related_formats: &[],
};
