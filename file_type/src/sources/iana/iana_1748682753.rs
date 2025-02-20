use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1748682753: FileType = FileType {
    file_format: &FileFormat {
        id: 1_748_682_753,
        source_type: SourceType::Iana,
        name: "vnd.laszip",
        extensions: &[],
        media_types: &["application/vnd.laszip"],
        signatures: &[],
        related_formats: &[],
    },
};
