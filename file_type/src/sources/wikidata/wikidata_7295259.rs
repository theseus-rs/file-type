use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7295259: FileType = FileType {
    file_format: &FileFormat {
        id: 7_295_259,
        source_type: SourceType::Wikidata,
        name: "Raster Document Object",
        extensions: &["rdo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
