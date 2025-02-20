use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1217839522: FileType = FileType {
    file_format: &FileFormat {
        id: 1_217_839_522,
        source_type: SourceType::Iana,
        name: "set-payment",
        extensions: &[],
        media_types: &["application/set-payment"],
        signatures: &[],
        related_formats: &[],
    },
};
