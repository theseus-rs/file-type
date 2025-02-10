use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129823013: FileType = FileType {
    file_format: &FileFormat {
        id: 129_823_013,
        source_type: SourceType::Wikidata,
        name: "Inform 7 source code file",
        extensions: &["i7x", "ni"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
