use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_336897408: FileType = FileType {
    file_format: &FileFormat {
        id: 336_897_408,
        source_type: SourceType::Iana,
        name: "vnd.longform",
        extensions: &[],
        media_types: &["text/vnd.longform"],
        signatures: &[],
        related_formats: &[],
    },
};
