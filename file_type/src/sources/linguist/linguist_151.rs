use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_151: FileType = FileType {
    file_format: &FileFormat {
        id: 151,
        source_type: SourceType::Linguist,
        name: "HTML+PHP",
        extensions: &["phtml"],
        media_types: &["application/x-httpd-php"],
        signatures: &[],
        related_formats: &[],
    },
};
