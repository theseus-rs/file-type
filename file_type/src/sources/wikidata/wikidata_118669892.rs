use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
