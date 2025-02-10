use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_48106028: FileType = FileType {
    file_format: &FileFormat {
        id: 48_106_028,
        source_type: SourceType::Wikidata,
        name: "Unisys (Sperry) System Data File",
        extensions: &["sdf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
