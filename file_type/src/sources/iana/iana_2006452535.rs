use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2006452535: FileType = FileType {
    file_format: &FileFormat {
        id: 2_006_452_535,
        source_type: SourceType::Iana,
        name: "tracking-status",
        extensions: &[],
        media_types: &["message/tracking-status"],
        signatures: &[],
        related_formats: &[],
    },
};
