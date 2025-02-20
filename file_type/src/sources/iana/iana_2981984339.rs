use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2981984339: FileType = FileType {
    file_format: &FileFormat {
        id: 2_981_984_339,
        source_type: SourceType::Iana,
        name: "vnd.powerbuilder6",
        extensions: &[],
        media_types: &["application/vnd.powerbuilder6"],
        signatures: &[],
        related_formats: &[],
    },
};
