use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1169399934: FileType = FileType {
    file_format: &FileFormat {
        id: 1_169_399_934,
        source_type: SourceType::Iana,
        name: "heic-sequence",
        extensions: &[],
        media_types: &["image/heic-sequence"],
        signatures: &[],
        related_formats: &[],
    },
};
