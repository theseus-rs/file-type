use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_65967080: FileType = FileType {
    file_format: &FileFormat {
        id: 65_967_080,
        source_type: SourceType::Wikidata,
        name: "Sketch file format",
        extensions: &["sketch"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
