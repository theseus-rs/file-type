use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3701836703: FileType = FileType {
    file_format: &FileFormat {
        id: 3_701_836_703,
        source_type: SourceType::Iana,
        name: "vnd.openstreetmap.data+xml",
        extensions: &[],
        media_types: &["application/vnd.openstreetmap.data+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
