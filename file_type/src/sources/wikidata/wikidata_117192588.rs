use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_117192588: FileType = FileType {
    file_format: &FileFormat {
        id: 117_192_588,
        source_type: SourceType::Wikidata,
        name: "Photoshop PDF",
        extensions: &["pdf", "pdp"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
