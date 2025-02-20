use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6717333: FileType = FileType {
    file_format: &FileFormat {
        id: 6_717_333,
        source_type: SourceType::Wikidata,
        name: "Mathematical Programming System",
        extensions: &["mps"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
