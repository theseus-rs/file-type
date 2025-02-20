use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_27979383: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_383,
        source_type: SourceType::Wikidata,
        name: "Heroglyph Project Format",
        extensions: &["hprj"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
