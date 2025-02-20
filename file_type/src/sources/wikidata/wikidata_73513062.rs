use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_73513062: FileType = FileType {
    file_format: &FileFormat {
        id: 73_513_062,
        source_type: SourceType::Wikidata,
        name: "Pathetic Writer document",
        extensions: &["pw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
