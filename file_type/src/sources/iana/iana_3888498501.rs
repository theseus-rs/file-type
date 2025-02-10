use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3888498501: FileType = FileType {
    file_format: &FileFormat {
        id: 3_888_498_501,
        source_type: SourceType::Iana,
        name: "vnd.etsi.iptvservice+xml",
        extensions: &[],
        media_types: &["application/vnd.etsi.iptvservice+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
