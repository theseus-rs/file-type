use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1692368822: FileType = FileType {
    file_format: &FileFormat {
        id: 1_692_368_822,
        source_type: SourceType::Iana,
        name: "vnd.genomatix.tuxedo",
        extensions: &[],
        media_types: &["application/vnd.genomatix.tuxedo"],
        signatures: &[],
        related_formats: &[],
    },
};
