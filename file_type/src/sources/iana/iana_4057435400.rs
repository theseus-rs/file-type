use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4057435400: FileType = FileType {
    file_format: &FileFormat {
        id: 4_057_435_400,
        source_type: SourceType::Iana,
        name: "prs.avid",
        extensions: &[],
        media_types: &["video/prs.avid"],
        signatures: &[],
        related_formats: &[],
    },
};
