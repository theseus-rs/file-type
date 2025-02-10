use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_6717026: FileType = FileType {
    file_format: &FileFormat {
        id: 6_717_026,
        source_type: SourceType::Wikidata,
        name: "MOI",
        extensions: &["moi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
