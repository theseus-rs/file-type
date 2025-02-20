use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117287768: FileType = FileType {
    file_format: &FileFormat {
        id: 117_287_768,
        source_type: SourceType::Wikidata,
        name: "SigmaPlot Template File",
        extensions: &["jnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
