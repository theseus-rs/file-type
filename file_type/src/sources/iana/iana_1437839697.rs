use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1437839697: FileType = FileType {
    file_format: &FileFormat {
        id: 1_437_839_697,
        source_type: SourceType::Iana,
        name: "LXF",
        extensions: &[],
        media_types: &["application/LXF"],
        signatures: &[],
        related_formats: &[],
    },
};
