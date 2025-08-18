use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_105849694: FileType = FileType {
    file_format: &FileFormat {
        id: 105_849_694,
        source_type: SourceType::Wikidata,
        name: "Celestia script",
        extensions: &["cel"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
