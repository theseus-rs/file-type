use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83276106: FileType = FileType {
    file_format: &FileFormat {
        id: 83_276_106,
        source_type: SourceType::Wikidata,
        name: "Interleaf/Quicksilver file format",
        extensions: &["ildoc"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
