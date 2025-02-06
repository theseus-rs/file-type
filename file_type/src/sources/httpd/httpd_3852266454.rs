use crate::format::{FileFormat, SourceType};

pub(crate) const HTTPD_3852266454: FileFormat = FileFormat {
    id: 3_852_266_454,
    source_type: SourceType::Httpd,
    name: "wordperfect",
    extensions: &["wpd"],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[],
    related_formats: &[],
};
