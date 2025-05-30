use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4178770917: FileType = FileType {
    file_format: &FileFormat {
        id: 4_178_770_917,
        source_type: SourceType::Iana,
        name: "L8",
        extensions: &[],
        media_types: &["audio/L8"],
        signatures: &[],
        related_formats: &[],
    },
};
