use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_79237925: FileType = FileType {
    file_format: &FileFormat {
        id: 79_237_925,
        source_type: SourceType::Wikidata,
        name: "Amapi 3D model",
        extensions: &["a3d"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
