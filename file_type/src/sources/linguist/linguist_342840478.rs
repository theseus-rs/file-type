use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const LINGUIST_342840478: FileType = FileType {
    file_format: &FileFormat {
        id: 342_840_478,
        source_type: SourceType::Linguist,
        name: "Edje Data Collection",
        extensions: &["edc"],
        media_types: &["text/x-c++src"],
        signatures: &[],
        related_formats: &[],
    },
};
