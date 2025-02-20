use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_130479459: FileType = FileType {
    file_format: &FileFormat {
        id: 130_479_459,
        source_type: SourceType::Wikidata,
        name: "Pony source code file",
        extensions: &["pony"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
