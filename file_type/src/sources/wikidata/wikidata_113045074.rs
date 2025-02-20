use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113045074: FileType = FileType {
    file_format: &FileFormat {
        id: 113_045_074,
        source_type: SourceType::Wikidata,
        name: "PTC G-Plug Granite file",
        extensions: &["g"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
