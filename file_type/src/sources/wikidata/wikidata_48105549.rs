use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_48105549: FileType = FileType {
    file_format: &FileFormat {
        id: 48_105_549,
        source_type: SourceType::Wikidata,
        name: "SAS for MS-DOS Catalog",
        extensions: &["sct"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
