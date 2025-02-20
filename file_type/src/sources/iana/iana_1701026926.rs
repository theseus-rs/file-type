use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1701026926: FileType = FileType {
    file_format: &FileFormat {
        id: 1_701_026_926,
        source_type: SourceType::Iana,
        name: "clr",
        extensions: &[],
        media_types: &["application/clr"],
        signatures: &[],
        related_formats: &[],
    },
};
