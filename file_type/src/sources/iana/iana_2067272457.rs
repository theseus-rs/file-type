use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_2067272457: FileType = FileType {
    file_format: &FileFormat {
        id: 2_067_272_457,
        source_type: SourceType::Iana,
        name: "vnd.cld",
        extensions: &[],
        media_types: &["model/vnd.cld"],
        signatures: &[],
        related_formats: &[],
    },
};
