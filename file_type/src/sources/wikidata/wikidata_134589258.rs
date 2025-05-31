use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_134589258: FileType = FileType {
    file_format: &FileFormat {
        id: 134_589_258,
        source_type: SourceType::Wikidata,
        name: "Pyspread save file",
        extensions: &["pysu"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
