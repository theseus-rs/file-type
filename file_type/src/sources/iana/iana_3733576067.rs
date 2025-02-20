use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3733576067: FileType = FileType {
    file_format: &FileFormat {
        id: 3_733_576_067,
        source_type: SourceType::Iana,
        name: "dns-message",
        extensions: &[],
        media_types: &["application/dns-message"],
        signatures: &[],
        related_formats: &[],
    },
};
