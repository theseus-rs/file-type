use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1080843780: FileType = FileType {
    file_format: &FileFormat {
        id: 1_080_843_780,
        source_type: SourceType::Iana,
        name: "vnd.wolfram.mathematica",
        extensions: &[],
        media_types: &["application/vnd.wolfram.mathematica"],
        signatures: &[],
        related_formats: &[],
    },
};
