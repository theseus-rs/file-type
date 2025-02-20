use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48912365: FileType = FileType {
    file_format: &FileFormat {
        id: 48_912_365,
        source_type: SourceType::Wikidata,
        name: "InterBase Database",
        extensions: &["gdb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
