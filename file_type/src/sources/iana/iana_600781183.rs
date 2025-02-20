use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_600781183: FileType = FileType {
    file_format: &FileFormat {
        id: 600_781_183,
        source_type: SourceType::Iana,
        name: "watcherinfo+xml",
        extensions: &[],
        media_types: &["application/watcherinfo+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
