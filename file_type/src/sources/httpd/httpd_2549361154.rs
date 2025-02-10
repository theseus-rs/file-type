use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const HTTPD_2549361154: FileType = FileType {
    file_format: &FileFormat {
        id: 2_549_361_154,
        source_type: SourceType::Httpd,
        name: "authorware seg",
        extensions: &["aas"],
        media_types: &["application/x-authorware-seg"],
        signatures: &[],
        related_formats: &[],
    },
};
