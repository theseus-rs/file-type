use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118669892: FileType = FileType {
    file_format: &FileFormat {
        id: 118_669_892,
        source_type: SourceType::Wikidata,
        name: "Layer Link File",
        extensions: &["clk"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
