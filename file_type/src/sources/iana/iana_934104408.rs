use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_934104408: FileType = FileType {
    file_format: &FileFormat {
        id: 934_104_408,
        source_type: SourceType::Iana,
        name: "JPEG",
        extensions: &[],
        media_types: &["video/JPEG"],
        signatures: &[],
        related_formats: &[],
    },
};
