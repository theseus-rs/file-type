use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
