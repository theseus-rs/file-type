use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487485: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_485,
        source_type: SourceType::Wikidata,
        name: "Shapefile spatial index of features part 1",
        extensions: &["fbn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
