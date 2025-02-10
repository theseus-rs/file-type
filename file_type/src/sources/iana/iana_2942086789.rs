use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2942086789: FileType = FileType {
    file_format: &FileFormat {
        id: 2_942_086_789,
        source_type: SourceType::Iana,
        name: "prs.pti",
        extensions: &[],
        media_types: &["image/prs.pti"],
        signatures: &[],
        related_formats: &[],
    },
};
