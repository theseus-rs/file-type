use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_25823631: FileType = FileType {
    file_format: &FileFormat {
        id: 25_823_631,
        source_type: SourceType::Wikidata,
        name: "MapCSS",
        extensions: &["mapcss"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
