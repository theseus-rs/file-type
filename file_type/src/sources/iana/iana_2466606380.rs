use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2466606380: FileType = FileType {
    file_format: &FileFormat {
        id: 2_466_606_380,
        source_type: SourceType::Iana,
        name: "vnd.hp-HPGL",
        extensions: &[],
        media_types: &["application/vnd.hp-HPGL"],
        signatures: &[],
        related_formats: &[],
    },
};
