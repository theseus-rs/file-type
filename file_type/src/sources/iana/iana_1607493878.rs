use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1607493878: FileType = FileType {
    file_format: &FileFormat {
        id: 1_607_493_878,
        source_type: SourceType::Iana,
        name: "MPV",
        extensions: &[],
        media_types: &["video/MPV"],
        signatures: &[],
        related_formats: &[],
    },
};
