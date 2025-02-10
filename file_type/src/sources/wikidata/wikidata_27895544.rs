use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27895544: FileType = FileType {
    file_format: &FileFormat {
        id: 27_895_544,
        source_type: SourceType::Wikidata,
        name: "ADX, version 2",
        extensions: &["adx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
