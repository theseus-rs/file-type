use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_136654161: FileType = FileType {
    file_format: &FileFormat {
        id: 136_654_161,
        source_type: SourceType::Wikidata,
        name: "Raku module file",
        extensions: &["rakumod"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
