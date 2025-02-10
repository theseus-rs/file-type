use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3251275829: FileType = FileType {
    file_format: &FileFormat {
        id: 3_251_275_829,
        source_type: SourceType::Iana,
        name: "vnd.FloGraphIt",
        extensions: &[],
        media_types: &["application/vnd.FloGraphIt"],
        signatures: &[],
        related_formats: &[],
    },
};
