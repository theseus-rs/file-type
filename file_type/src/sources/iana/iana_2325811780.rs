use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_2325811780: FileType = FileType {
    file_format: &FileFormat {
        id: 2_325_811_780,
        source_type: SourceType::Iana,
        name: "senml-etch+json",
        extensions: &[],
        media_types: &["application/senml-etch+json"],
        signatures: &[],
        related_formats: &[],
    },
};
