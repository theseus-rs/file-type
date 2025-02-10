use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2477907789: FileType = FileType {
    file_format: &FileFormat {
        id: 2_477_907_789,
        source_type: SourceType::Iana,
        name: "vnd.lotus-notes",
        extensions: &[],
        media_types: &["application/vnd.lotus-notes"],
        signatures: &[],
        related_formats: &[],
    },
};
