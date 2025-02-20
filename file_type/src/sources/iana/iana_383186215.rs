use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
