use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const IANA_1750670724: FileType = FileType {
    file_format: &FileFormat {
        id: 1_750_670_724,
        source_type: SourceType::Iana,
        name: "vnd.hzn-3d-crossword",
        extensions: &[],
        media_types: &["application/vnd.hzn-3d-crossword"],
        signatures: &[],
        related_formats: &[],
    },
};
