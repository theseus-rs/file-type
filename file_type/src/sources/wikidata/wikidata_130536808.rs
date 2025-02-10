use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_130536808: FileType = FileType {
    file_format: &FileFormat {
        id: 130_536_808,
        source_type: SourceType::Wikidata,
        name: "PRQL source code file",
        extensions: &["prql"],
        media_types: &["application/prql", "application/x-prql"],
        signatures: &[],
        related_formats: &[],
    },
};
