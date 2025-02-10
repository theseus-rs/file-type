use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_484977096: FileType = FileType {
    file_format: &FileFormat {
        id: 484_977_096,
        source_type: SourceType::Iana,
        name: "dns",
        extensions: &[],
        media_types: &["text/dns"],
        signatures: &[],
        related_formats: &[],
    },
};
