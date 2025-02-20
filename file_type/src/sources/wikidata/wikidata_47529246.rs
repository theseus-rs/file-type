use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47529246: FileType = FileType {
    file_format: &FileFormat {
        id: 47_529_246,
        source_type: SourceType::Wikidata,
        name: "SuperScape Virtual Reality Format",
        extensions: &["svr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
