use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28207574: FileType = FileType {
    file_format: &FileFormat {
        id: 28_207_574,
        source_type: SourceType::Wikidata,
        name: "Zoomify PFF",
        extensions: &["pff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
