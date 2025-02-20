use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_125172737: FileType = FileType {
    file_format: &FileFormat {
        id: 125_172_737,
        source_type: SourceType::Wikidata,
        name: "MyNotex file",
        extensions: &["mnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
