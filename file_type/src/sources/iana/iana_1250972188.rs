use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1250972188: FileType = FileType {
    file_format: &FileFormat {
        id: 1_250_972_188,
        source_type: SourceType::Iana,
        name: "scvp-vp-response",
        extensions: &[],
        media_types: &["application/scvp-vp-response"],
        signatures: &[],
        related_formats: &[],
    },
};
