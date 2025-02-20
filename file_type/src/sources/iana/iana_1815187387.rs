use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1815187387: FileType = FileType {
    file_format: &FileFormat {
        id: 1_815_187_387,
        source_type: SourceType::Iana,
        name: "vnd.datapackage+json",
        extensions: &[],
        media_types: &["application/vnd.datapackage+json"],
        signatures: &[],
        related_formats: &[],
    },
};
