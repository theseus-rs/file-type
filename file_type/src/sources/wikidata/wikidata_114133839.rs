use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_114133839: FileType = FileType {
    file_format: &FileFormat {
        id: 114_133_839,
        source_type: SourceType::Wikidata,
        name: "MacroModels file format",
        extensions: &["mcm"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
