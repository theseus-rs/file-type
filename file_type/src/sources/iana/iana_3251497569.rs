use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3251497569: FileType = FileType {
    file_format: &FileFormat {
        id: 3_251_497_569,
        source_type: SourceType::Iana,
        name: "vnd.nacamar.ybrid+json",
        extensions: &[],
        media_types: &["application/vnd.nacamar.ybrid+json"],
        signatures: &[],
        related_formats: &[],
    },
};
