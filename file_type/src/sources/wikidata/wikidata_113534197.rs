use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113534197: FileType = FileType {
    file_format: &FileFormat {
        id: 113_534_197,
        source_type: SourceType::Wikidata,
        name: "Capture One Settings File",
        extensions: &["cos"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
