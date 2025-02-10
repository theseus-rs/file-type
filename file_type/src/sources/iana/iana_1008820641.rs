use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1008820641: FileType = FileType {
    file_format: &FileFormat {
        id: 1_008_820_641,
        source_type: SourceType::Iana,
        name: "PCMU-WB",
        extensions: &[],
        media_types: &["audio/PCMU-WB"],
        signatures: &[],
        related_formats: &[],
    },
};
