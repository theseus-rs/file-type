use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3352102566: FileType = FileType {
    file_format: &FileFormat {
        id: 3_352_102_566,
        source_type: SourceType::Iana,
        name: "H266",
        extensions: &[],
        media_types: &["video/H266"],
        signatures: &[],
        related_formats: &[],
    },
};
