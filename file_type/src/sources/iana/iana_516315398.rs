use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
