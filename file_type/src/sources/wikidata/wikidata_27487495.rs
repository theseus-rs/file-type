use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487495: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_495,
        source_type: SourceType::Wikidata,
        name: "Shapefile spatial index of features part 2",
        extensions: &["fbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
