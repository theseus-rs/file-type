use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2205754082: FileType = FileType {
    file_format: &FileFormat {
        id: 2_205_754_082,
        source_type: SourceType::Iana,
        name: "xacml+xml",
        extensions: &[],
        media_types: &["application/xacml+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
