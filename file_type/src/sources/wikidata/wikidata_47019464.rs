use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47019464: FileType = FileType {
    file_format: &FileFormat {
        id: 47_019_464,
        source_type: SourceType::Wikidata,
        name: "PageMaker Document file format, version 5",
        extensions: &["pm5"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
