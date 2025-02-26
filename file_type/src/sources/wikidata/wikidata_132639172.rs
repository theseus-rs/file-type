use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_132639172: FileType = FileType {
    file_format: &FileFormat {
        id: 132_639_172,
        source_type: SourceType::Wikidata,
        name: "Scriptol source file",
        extensions: &["sol"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
