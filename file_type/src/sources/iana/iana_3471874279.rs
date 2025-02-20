use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3471874279: FileType = FileType {
    file_format: &FileFormat {
        id: 3_471_874_279,
        source_type: SourceType::Iana,
        name: "jsonpath",
        extensions: &[],
        media_types: &["application/jsonpath"],
        signatures: &[],
        related_formats: &[],
    },
};
