use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206657: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_657,
        source_type: SourceType::Wikidata,
        name: "Nero CoverDesigner Document",
        extensions: &["ncd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
