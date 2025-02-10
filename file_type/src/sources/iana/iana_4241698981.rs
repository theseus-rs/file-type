use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4241698981: FileType = FileType {
    file_format: &FileFormat {
        id: 4_241_698_981,
        source_type: SourceType::Iana,
        name: "set-registration",
        extensions: &[],
        media_types: &["application/set-registration"],
        signatures: &[],
        related_formats: &[],
    },
};
