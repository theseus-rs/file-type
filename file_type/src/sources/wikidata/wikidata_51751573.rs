use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_51751573: FileType = FileType {
    file_format: &FileFormat {
        id: 51_751_573,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Script",
        extensions: &["scr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
