use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3461016671: FileType = FileType {
    file_format: &FileFormat {
        id: 3_461_016_671,
        source_type: SourceType::Iana,
        name: "vnd.hc+json",
        extensions: &[],
        media_types: &["application/vnd.hc+json"],
        signatures: &[],
        related_formats: &[],
    },
};
