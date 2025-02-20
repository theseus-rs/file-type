use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27487130: FileType = FileType {
    file_format: &FileFormat {
        id: 27_487_130,
        source_type: SourceType::Wikidata,
        name: "Shapefile dBASE table",
        extensions: &["dbf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
