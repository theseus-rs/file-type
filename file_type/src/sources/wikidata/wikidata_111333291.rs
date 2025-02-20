use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111333291: FileType = FileType {
    file_format: &FileFormat {
        id: 111_333_291,
        source_type: SourceType::Wikidata,
        name: "DisorderTracker2 sample",
        extensions: &["pls"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
