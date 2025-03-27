use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_113396644: FileType = FileType {
    file_format: &FileFormat {
        id: 113_396_644,
        source_type: SourceType::Wikidata,
        name: "Graphical Pathway Markup Language",
        extensions: &["gpml"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
