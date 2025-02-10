use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1236131182: FileType = FileType {
    file_format: &FileFormat {
        id: 1_236_131_182,
        source_type: SourceType::Iana,
        name: "vnd.joost.joda-archive",
        extensions: &[],
        media_types: &["application/vnd.joost.joda-archive"],
        signatures: &[],
        related_formats: &[],
    },
};
