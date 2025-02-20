use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_119846012: FileType = FileType {
    file_format: &FileFormat {
        id: 119_846_012,
        source_type: SourceType::Wikidata,
        name: "Quicken Data File (Macintosh)",
        extensions: &["qdfm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
