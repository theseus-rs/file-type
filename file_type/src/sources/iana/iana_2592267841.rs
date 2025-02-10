use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2592267841: FileType = FileType {
    file_format: &FileFormat {
        id: 2_592_267_841,
        source_type: SourceType::Iana,
        name: "sarif-external-properties+json",
        extensions: &[],
        media_types: &["application/sarif-external-properties+json"],
        signatures: &[],
        related_formats: &[],
    },
};
