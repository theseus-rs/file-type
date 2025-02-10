use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3065629133: FileType = FileType {
    file_format: &FileFormat {
        id: 3_065_629_133,
        source_type: SourceType::Iana,
        name: "vnd.cloanto.rp9",
        extensions: &[],
        media_types: &["application/vnd.cloanto.rp9"],
        signatures: &[],
        related_formats: &[],
    },
};
