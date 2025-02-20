use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2584743258: FileType = FileType {
    file_format: &FileFormat {
        id: 2_584_743_258,
        source_type: SourceType::Iana,
        name: "AML",
        extensions: &[],
        media_types: &["application/AML"],
        signatures: &[],
        related_formats: &[],
    },
};
