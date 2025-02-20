use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487522: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_522,
        source_type: SourceType::Wikidata,
        name: "Shapefile attribute index part 1",
        extensions: &["ain"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
