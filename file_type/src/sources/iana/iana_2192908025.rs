use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2192908025: FileType = FileType {
    file_format: &FileFormat {
        id: 2_192_908_025,
        source_type: SourceType::Iana,
        name: "prs.sclt",
        extensions: &[],
        media_types: &["application/prs.sclt"],
        signatures: &[],
        related_formats: &[],
    },
};
