use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_52063275: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_275,
        source_type: SourceType::Wikidata,
        name: "Professional Write Text File",
        extensions: &["pw"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
