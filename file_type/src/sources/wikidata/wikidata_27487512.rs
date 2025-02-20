use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487512: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_512,
        source_type: SourceType::Wikidata,
        name: "Shapefile header index",
        extensions: &["aih"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
