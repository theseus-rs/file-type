use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1888920778: FileType = FileType {
    file_format: &FileFormat {
        id: 1_888_920_778,
        source_type: SourceType::Iana,
        name: "evc",
        extensions: &[],
        media_types: &["video/evc"],
        signatures: &[],
        related_formats: &[],
    },
};
