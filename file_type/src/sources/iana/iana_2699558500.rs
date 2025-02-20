use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2699558500: FileType = FileType {
    file_format: &FileFormat {
        id: 2_699_558_500,
        source_type: SourceType::Iana,
        name: "vnd.hp-hps",
        extensions: &[],
        media_types: &["application/vnd.hp-hps"],
        signatures: &[],
        related_formats: &[],
    },
};
