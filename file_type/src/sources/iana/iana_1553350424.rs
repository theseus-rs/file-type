use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1553350424: FileType = FileType {
    file_format: &FileFormat {
        id: 1_553_350_424,
        source_type: SourceType::Iana,
        name: "vnd.apache.parquet",
        extensions: &[],
        media_types: &["application/vnd.apache.parquet"],
        signatures: &[],
        related_formats: &[],
    },
};
