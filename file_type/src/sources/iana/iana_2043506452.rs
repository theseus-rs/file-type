use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2043506452: FileType = FileType {
    file_format: &FileFormat {
        id: 2_043_506_452,
        source_type: SourceType::Iana,
        name: "vnd.wfa.dpp",
        extensions: &[],
        media_types: &["application/vnd.wfa.dpp"],
        signatures: &[],
        related_formats: &[],
    },
};
