use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3039503997: FileType = FileType {
    file_format: &FileFormat {
        id: 3_039_503_997,
        source_type: SourceType::Iana,
        name: "link-format",
        extensions: &[],
        media_types: &["application/link-format"],
        signatures: &[],
        related_formats: &[],
    },
};
