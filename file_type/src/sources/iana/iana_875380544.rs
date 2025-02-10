use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_875380544: FileType = FileType {
    file_format: &FileFormat {
        id: 875_380_544,
        source_type: SourceType::Iana,
        name: "DII",
        extensions: &[],
        media_types: &["application/DII"],
        signatures: &[],
        related_formats: &[],
    },
};
