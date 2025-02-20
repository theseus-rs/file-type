use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1312725: FileType = FileType {
    file_format: &FileFormat {
        id: 1_312_725,
        source_type: SourceType::Wikidata,
        name: "local shared object",
        extensions: &["sol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
