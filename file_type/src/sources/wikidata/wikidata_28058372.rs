use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28058372: FileType = FileType {
    file_format: &FileFormat {
        id: 28_058_372,
        source_type: SourceType::Wikidata,
        name: "IFF-FAXX",
        extensions: &["faxx"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
