use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_4061354234: FileType = FileType {
    file_format: &FileFormat {
        id: 4_061_354_234,
        source_type: SourceType::Iana,
        name: "vnd.sybyl.mol2",
        extensions: &[],
        media_types: &["application/vnd.sybyl.mol2"],
        signatures: &[],
        related_formats: &[],
    },
};
