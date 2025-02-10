use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3225988671: FileType = FileType {
    file_format: &FileFormat {
        id: 3_225_988_671,
        source_type: SourceType::Iana,
        name: "3gpp2",
        extensions: &[],
        media_types: &["video/3gpp2"],
        signatures: &[],
        related_formats: &[],
    },
};
