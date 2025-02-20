use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_118640906: FileType = FileType {
    file_format: &FileFormat {
        id: 118_640_906,
        source_type: SourceType::Wikidata,
        name: "Manga Studio 3D Object",
        extensions: &["cso"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
