use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_111666304: FileType = FileType {
    file_format: &FileFormat {
        id: 111_666_304,
        source_type: SourceType::Wikidata,
        name: "Liveart Sketches",
        extensions: &["lrt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
