use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1848133025: FileType = FileType {
    file_format: &FileFormat {
        id: 1_848_133_025,
        source_type: SourceType::Iana,
        name: "x3d-vrml",
        extensions: &[],
        media_types: &["model/x3d-vrml"],
        signatures: &[],
        related_formats: &[],
    },
};
