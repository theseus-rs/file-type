use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_50498413: FileType = FileType {
    file_format: &FileFormat {
        id: 50_498_413,
        source_type: SourceType::Wikidata,
        name: "Draco File Format",
        extensions: &["drc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
