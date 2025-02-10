use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_18653981: FileType = FileType {
    file_format: &FileFormat {
        id: 18_653_981,
        source_type: SourceType::Wikidata,
        name: "Standard Delay Format",
        extensions: &["sdf", "sdo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
