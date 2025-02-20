use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206159: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_159,
        source_type: SourceType::Wikidata,
        name: "G9B",
        extensions: &["g9b"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
