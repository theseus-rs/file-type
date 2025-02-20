use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111521610: FileType = FileType {
    file_format: &FileFormat {
        id: 111_521_610,
        source_type: SourceType::Wikidata,
        name: "Packed-Ice True Colour Sprites",
        extensions: &["trs"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
