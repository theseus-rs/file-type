use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_1404343267: FileType = FileType {
    file_format: &FileFormat {
        id: 1_404_343_267,
        source_type: SourceType::Iana,
        name: "sep-exi",
        extensions: &[],
        media_types: &["application/sep-exi"],
        signatures: &[],
        related_formats: &[],
    },
};
