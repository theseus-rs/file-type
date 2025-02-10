use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3337385297: FileType = FileType {
    file_format: &FileFormat {
        id: 3_337_385_297,
        source_type: SourceType::Iana,
        name: "timestamped-data",
        extensions: &[],
        media_types: &["application/timestamped-data"],
        signatures: &[],
        related_formats: &[],
    },
};
