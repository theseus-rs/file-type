use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487343: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_343,
        source_type: SourceType::Wikidata,
        name: "Shapefile spatial index part 1",
        extensions: &["sbn"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
