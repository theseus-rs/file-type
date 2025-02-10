use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47019065: FileType = FileType {
    file_format: &FileFormat {
        id: 47_019_065,
        source_type: SourceType::Wikidata,
        name: "PageMaker Document file format, version 6",
        extensions: &["pm6"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
