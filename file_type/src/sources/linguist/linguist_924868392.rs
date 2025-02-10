use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const LINGUIST_924868392: FileType = FileType {
    file_format: &FileFormat {
        id: 924_868_392,
        source_type: SourceType::Linguist,
        name: "OMNeT++ NED",
        extensions: &["ned"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
