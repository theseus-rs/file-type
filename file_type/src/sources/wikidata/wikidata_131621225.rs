use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131621225: FileType = FileType {
    file_format: &FileFormat {
        id: 131_621_225,
        source_type: SourceType::Wikidata,
        name: "Dyna database file format",
        extensions: &["d3plot"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
