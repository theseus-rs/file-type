use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_52063393: FileType = FileType {
    file_format: &FileFormat {
        id: 52_063_393,
        source_type: SourceType::Wikidata,
        name: "TeX Binary File",
        extensions: &["dvi"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
