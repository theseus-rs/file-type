use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_159891324: FileType = FileType {
    file_format: &FileFormat {
        id: 159_891_324,
        source_type: SourceType::Iana,
        name: "msc-ivr+xml",
        extensions: &[],
        media_types: &["application/msc-ivr+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
