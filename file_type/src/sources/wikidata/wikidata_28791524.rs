use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28791524: FileType = FileType {
    file_format: &FileFormat {
        id: 28_791_524,
        source_type: SourceType::Wikidata,
        name: "Parchive, version 1",
        extensions: &["par"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
