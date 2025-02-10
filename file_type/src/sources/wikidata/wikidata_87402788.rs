use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_87402788: FileType = FileType {
    file_format: &FileFormat {
        id: 87_402_788,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Temporary File",
        extensions: &["ac$"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
