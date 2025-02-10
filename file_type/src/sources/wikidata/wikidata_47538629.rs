use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47538629: FileType = FileType {
    file_format: &FileFormat {
        id: 47_538_629,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Colour-Dependant Plot Style Table",
        extensions: &["ctb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
