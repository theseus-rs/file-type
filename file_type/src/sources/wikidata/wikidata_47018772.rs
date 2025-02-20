use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47018772: FileType = FileType {
    file_format: &FileFormat {
        id: 47_018_772,
        source_type: SourceType::Wikidata,
        name: "PageMaker Document file format, version 6.5",
        extensions: &["p65"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
