use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3433583701: FileType = FileType {
    file_format: &FileFormat {
        id: 3_433_583_701,
        source_type: SourceType::Iana,
        name: "vnd.eclipse.ditto+json",
        extensions: &[],
        media_types: &["application/vnd.eclipse.ditto+json"],
        signatures: &[],
        related_formats: &[],
    },
};
