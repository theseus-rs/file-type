use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_533608977: FileType = FileType {
    file_format: &FileFormat {
        id: 533_608_977,
        source_type: SourceType::Iana,
        name: "patch-ops-error+xml",
        extensions: &[],
        media_types: &["application/patch-ops-error+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
