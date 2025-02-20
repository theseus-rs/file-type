use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_3467076426: FileType = FileType {
    file_format: &FileFormat {
        id: 3_467_076_426,
        source_type: SourceType::Iana,
        name: "parityfec",
        extensions: &[],
        media_types: &["application/parityfec"],
        signatures: &[],
        related_formats: &[],
    },
};
