use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110984425: FileType = FileType {
    file_format: &FileFormat {
        id: 110_984_425,
        source_type: SourceType::Wikidata,
        name: "Video ToolBox 2 Project file",
        extensions: &["vtp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
