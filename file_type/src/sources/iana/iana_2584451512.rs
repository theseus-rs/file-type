use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2584451512: FileType = FileType {
    file_format: &FileFormat {
        id: 2_584_451_512,
        source_type: SourceType::Iana,
        name: "PCMA-WB",
        extensions: &[],
        media_types: &["audio/PCMA-WB"],
        signatures: &[],
        related_formats: &[],
    },
};
