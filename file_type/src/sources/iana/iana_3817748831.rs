use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3817748831: FileType = FileType {
    file_format: &FileFormat {
        id: 3_817_748_831,
        source_type: SourceType::Iana,
        name: "vnd.liberty-request+xml",
        extensions: &[],
        media_types: &["application/vnd.liberty-request+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
