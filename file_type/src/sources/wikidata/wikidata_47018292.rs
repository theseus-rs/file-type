use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_47018292: FileType = FileType {
    file_format: &FileFormat {
        id: 47_018_292,
        source_type: SourceType::Wikidata,
        name: "PageMaker Document file format, version 3",
        extensions: &["pm3"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
