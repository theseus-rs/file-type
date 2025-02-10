use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130267688: FileType = FileType {
    file_format: &FileFormat {
        id: 130_267_688,
        source_type: SourceType::Wikidata,
        name: "STL file format",
        extensions: &["stl"],
        media_types: &["model/stl"],
        signatures: &[],
        related_formats: &[],
    },
};
