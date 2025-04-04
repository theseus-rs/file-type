use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_6717445: FileType = FileType {
    file_format: &FileFormat {
        id: 6_717_445,
        source_type: SourceType::Wikidata,
        name: "MRC",
        extensions: &["mrc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
