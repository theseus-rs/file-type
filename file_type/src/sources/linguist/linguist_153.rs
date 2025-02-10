use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_153: FileType = FileType {
    file_format: &FileFormat {
        id: 153,
        source_type: SourceType::Linguist,
        name: "Hack",
        extensions: &["hack", "hh", "hhi", "php"],
        media_types: &["application/x-httpd-php"],
        signatures: &[],
        related_formats: &[],
    },
};
