use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_272: FileType = FileType {
    file_format: &FileFormat {
        id: 272,
        source_type: SourceType::Linguist,
        name: "PHP",
        extensions: &[
            "aw", "ctp", "fcgi", "inc", "php", "php3", "php4", "php5", "phps", "phpt",
        ],
        media_types: &["application/x-httpd-php"],
        signatures: &[],
        related_formats: &[],
    },
};
