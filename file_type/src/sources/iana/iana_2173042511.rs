use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2173042511: FileType = FileType {
    file_format: &FileFormat {
        id: 2_173_042_511,
        source_type: SourceType::Iana,
        name: "rpki-checklist",
        extensions: &[],
        media_types: &["application/rpki-checklist"],
        signatures: &[],
        related_formats: &[],
    },
};
