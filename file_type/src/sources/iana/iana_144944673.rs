use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_144944673: FileType = FileType {
    file_format: &FileFormat {
        id: 144_944_673,
        source_type: SourceType::Iana,
        name: "vnd.edulith.edux+json",
        extensions: &[],
        media_types: &["application/vnd.edulith.edux+json"],
        signatures: &[],
        related_formats: &[],
    },
};
