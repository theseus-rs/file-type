use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_3112925738: FileType = FileType {
    file_format: &FileFormat {
        id: 3_112_925_738,
        source_type: SourceType::Iana,
        name: "lgr+xml",
        extensions: &[],
        media_types: &["application/lgr+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
