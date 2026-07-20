use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_879593259: FileType = FileType {
    file_format: &FileFormat {
        id: 879_593_259,
        source_type: SourceType::Iana,
        name: "measured-component+json",
        extensions: &[],
        media_types: &["application/measured-component+json"],
        signatures: &[],
        related_formats: &[],
    },
};
