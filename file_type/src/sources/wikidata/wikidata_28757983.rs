use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28757983: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_983,
        source_type: SourceType::Wikidata,
        name: "IPF",
        extensions: &["ipf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
