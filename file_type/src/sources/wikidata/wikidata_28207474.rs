use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28207474: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_474,
        source_type: SourceType::Wikidata,
        name: "Very Ordinary Rendering Toolkit file",
        extensions: &["pix"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
