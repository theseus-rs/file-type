use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111009551: FileType = FileType {
    file_format: &FileFormat {
        id: 111_009_551,
        source_type: SourceType::Wikidata,
        name: "PrintMaster Craft File format",
        extensions: &["cft"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
