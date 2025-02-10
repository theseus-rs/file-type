use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_117192692: FileType = FileType {
    file_format: &FileFormat {
        id: 117_192_692,
        source_type: SourceType::Wikidata,
        name: "Photoshop Raw",
        extensions: &["raw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
