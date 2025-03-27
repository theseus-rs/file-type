use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_7389358: FileType = FileType {
    file_format: &FileFormat {
        id: 7_389_358,
        source_type: SourceType::Wikidata,
        name: "State Chart XML",
        extensions: &["scxml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
