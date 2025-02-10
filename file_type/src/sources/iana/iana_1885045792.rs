use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1885045792: FileType = FileType {
    file_format: &FileFormat {
        id: 1_885_045_792,
        source_type: SourceType::Iana,
        name: "prs.cyn",
        extensions: &[],
        media_types: &["application/prs.cyn"],
        signatures: &[],
        related_formats: &[],
    },
};
