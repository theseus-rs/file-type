use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28347778: FileType = FileType {
    file_format: &FileFormat {
        id: 28_347_778,
        source_type: SourceType::Wikidata,
        name: "Zeno",
        extensions: &["zeno"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
