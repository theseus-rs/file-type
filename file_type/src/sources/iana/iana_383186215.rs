use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_383186215: FileType = FileType {
    file_format: &FileFormat {
        id: 383_186_215,
        source_type: SourceType::Iana,
        name: "vc",
        extensions: &[],
        media_types: &["application/vc"],
        signatures: &[],
        related_formats: &[],
    },
};
