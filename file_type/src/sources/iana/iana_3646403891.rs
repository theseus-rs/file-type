use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3646403891: FileType = FileType {
    file_format: &FileFormat {
        id: 3_646_403_891,
        source_type: SourceType::Iana,
        name: "vemmi",
        extensions: &[],
        media_types: &["application/vemmi"],
        signatures: &[],
        related_formats: &[],
    },
};
