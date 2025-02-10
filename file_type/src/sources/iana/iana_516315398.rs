use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_516315398: FileType = FileType {
    file_format: &FileFormat {
        id: 516_315_398,
        source_type: SourceType::Iana,
        name: "ivs",
        extensions: &[],
        media_types: &["haptics/ivs"],
        signatures: &[],
        related_formats: &[],
    },
};
