use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2037854732: FileType = FileType {
    file_format: &FileFormat {
        id: 2_037_854_732,
        source_type: SourceType::Iana,
        name: "prs.xsf+xml",
        extensions: &[],
        media_types: &["application/prs.xsf+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
