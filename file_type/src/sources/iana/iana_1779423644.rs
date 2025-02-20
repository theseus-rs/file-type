use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1779423644: FileType = FileType {
    file_format: &FileFormat {
        id: 1_779_423_644,
        source_type: SourceType::Iana,
        name: "commonground",
        extensions: &[],
        media_types: &["application/commonground"],
        signatures: &[],
        related_formats: &[],
    },
};
