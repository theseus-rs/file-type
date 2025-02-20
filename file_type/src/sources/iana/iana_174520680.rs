use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_174520680: FileType = FileType {
    file_format: &FileFormat {
        id: 174_520_680,
        source_type: SourceType::Iana,
        name: "TETRA_ACELP",
        extensions: &[],
        media_types: &["audio/TETRA_ACELP"],
        signatures: &[],
        related_formats: &[],
    },
};
