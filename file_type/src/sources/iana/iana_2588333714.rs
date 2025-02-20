use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2588333714: FileType = FileType {
    file_format: &FileFormat {
        id: 2_588_333_714,
        source_type: SourceType::Iana,
        name: "sgml-open-catalog",
        extensions: &[],
        media_types: &["application/sgml-open-catalog"],
        signatures: &[],
        related_formats: &[],
    },
};
