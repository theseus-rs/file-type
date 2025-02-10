use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_80973015: FileType = FileType {
    file_format: &FileFormat {
        id: 80_973_015,
        source_type: SourceType::Iana,
        name: "vnd.oasis.opendocument.presentation",
        extensions: &[],
        media_types: &["application/vnd.oasis.opendocument.presentation"],
        signatures: &[],
        related_formats: &[],
    },
};
