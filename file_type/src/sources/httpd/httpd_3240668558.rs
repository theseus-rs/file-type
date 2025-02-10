use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_3240668558: FileType = FileType {
    file_format: &FileFormat {
        id: 3_240_668_558,
        source_type: SourceType::Httpd,
        name: "spotfire sfs",
        extensions: &["sfs"],
        media_types: &["application/vnd.spotfire.sfs"],
        signatures: &[],
        related_formats: &[],
    },
};
