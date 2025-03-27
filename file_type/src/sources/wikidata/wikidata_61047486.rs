use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_61047486: FileType = FileType {
    file_format: &FileFormat {
        id: 61_047_486,
        source_type: SourceType::Wikidata,
        name: "Biological Expression Language",
        extensions: &["bel"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
