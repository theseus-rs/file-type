use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27487348: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_348,
        source_type: SourceType::Wikidata,
        name: "Shapefile spatial index part 2",
        extensions: &["sbx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
