use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_151: FileFormat = FileFormat {
    id: 151,
    source_type: SourceType::Linguist,
    name: "HTML+PHP",
    extensions: &["phtml"],
    media_types: &["application/x-httpd-php"],
    signatures: &[],
    related_formats: &[],
};
