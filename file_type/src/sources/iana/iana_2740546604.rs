use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2740546604: FileType = FileType {
    file_format: &FileFormat {
        id: 2_740_546_604,
        source_type: SourceType::Iana,
        name: "cose",
        extensions: &[],
        media_types: &["application/cose"],
        signatures: &[],
        related_formats: &[],
    },
};
