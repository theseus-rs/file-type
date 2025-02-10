use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3519971946: FileType = FileType {
    file_format: &FileFormat {
        id: 3_519_971_946,
        source_type: SourceType::Iana,
        name: "mmt-aei+xml",
        extensions: &[],
        media_types: &["application/mmt-aei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
