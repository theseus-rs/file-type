use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_27979408: FileType = FileType {
    file_format: &FileFormat {
        id: 27_979_408,
        source_type: SourceType::Wikidata,
        name: "XNG",
        extensions: &["xng"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
