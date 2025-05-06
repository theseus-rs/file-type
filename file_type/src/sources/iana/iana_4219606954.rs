use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const IANA_4219606954: FileType = FileType {
    file_format: &FileFormat {
        id: 4_219_606_954,
        source_type: SourceType::Iana,
        name: "texinfo",
        extensions: &[],
        media_types: &["application/texinfo"],
        signatures: &[],
        related_formats: &[],
    },
};
