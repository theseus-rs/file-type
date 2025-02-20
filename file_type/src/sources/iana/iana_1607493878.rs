use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
