use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1294551089: FileType = FileType {
    file_format: &FileFormat {
        id: 1_294_551_089,
        source_type: SourceType::Iana,
        name: "EVRC1",
        extensions: &[],
        media_types: &["audio/EVRC1"],
        signatures: &[],
        related_formats: &[],
    },
};
