use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28049588: FileType = FileType {
    file_format: &FileFormat {
        id: 28_049_588,
        source_type: SourceType::Wikidata,
        name: "Tiny Stuff, low resolution",
        extensions: &["tn1", "tny"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
