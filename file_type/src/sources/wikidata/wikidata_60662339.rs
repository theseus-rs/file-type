use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_60662339: FileType = FileType {
    file_format: &FileFormat {
        id: 60_662_339,
        source_type: SourceType::Wikidata,
        name: "AutoCAD Plot Configuration File, version 1",
        extensions: &["pcp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
