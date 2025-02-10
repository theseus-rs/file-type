use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130726128: FileType = FileType {
    file_format: &FileFormat {
        id: 130_726_128,
        source_type: SourceType::Wikidata,
        name: "S source code file",
        extensions: &["S"],
        media_types: &["text/S"],
        signatures: &[],
        related_formats: &[],
    },
};
