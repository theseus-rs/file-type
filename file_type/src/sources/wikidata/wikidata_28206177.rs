use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28206177: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_177,
        source_type: SourceType::Wikidata,
        name: "GIMP Brush",
        extensions: &["gbr", "gpb"],
        media_types: &["image/x-gimp-gbr"],
        signatures: &[],
        related_formats: &[],
    },
};
