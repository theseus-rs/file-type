use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600712: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_712,
        source_type: SourceType::Wikidata,
        name: "DoItAgain",
        extensions: &["dia"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
