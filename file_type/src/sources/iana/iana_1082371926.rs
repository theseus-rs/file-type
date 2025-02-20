use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1082371926: FileType = FileType {
    file_format: &FileFormat {
        id: 1_082_371_926,
        source_type: SourceType::Iana,
        name: "vnd.gov.sk.e-form+xml (OBSOLETED by request)",
        extensions: &[],
        media_types: &["application/vnd.gov.sk.e-form+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
