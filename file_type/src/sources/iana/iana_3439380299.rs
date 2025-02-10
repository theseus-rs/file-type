use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3439380299: FileType = FileType {
    file_format: &FileFormat {
        id: 3_439_380_299,
        source_type: SourceType::Iana,
        name: "vnd.apache.arrow.stream",
        extensions: &[],
        media_types: &["application/vnd.apache.arrow.stream"],
        signatures: &[],
        related_formats: &[],
    },
};
