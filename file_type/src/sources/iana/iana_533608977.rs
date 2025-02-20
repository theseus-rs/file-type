use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
