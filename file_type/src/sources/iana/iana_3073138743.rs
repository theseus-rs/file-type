use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3073138743: FileType = FileType {
    file_format: &FileFormat {
        id: 3_073_138_743,
        source_type: SourceType::Iana,
        name: "EVRC0",
        extensions: &[],
        media_types: &["audio/EVRC0"],
        signatures: &[],
        related_formats: &[],
    },
};
