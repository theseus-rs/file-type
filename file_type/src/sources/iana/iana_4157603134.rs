use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4157603134: FileType = FileType {
    file_format: &FileFormat {
        id: 4_157_603_134,
        source_type: SourceType::Iana,
        name: "PCMU",
        extensions: &[],
        media_types: &["audio/PCMU"],
        signatures: &[],
        related_formats: &[],
    },
};
