use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2377153967: FileType = FileType {
    file_format: &FileFormat {
        id: 2_377_153_967,
        source_type: SourceType::Iana,
        name: "IOTP",
        extensions: &[],
        media_types: &["application/IOTP"],
        signatures: &[],
        related_formats: &[],
    },
};
