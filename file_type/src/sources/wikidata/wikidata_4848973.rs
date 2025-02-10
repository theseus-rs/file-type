use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_4848973: FileType = FileType {
    file_format: &FileFormat {
        id: 4_848_973,
        source_type: SourceType::Wikidata,
        name: "Bak file",
        extensions: &["bak"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
