use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_111368241: FileType = FileType {
    file_format: &FileFormat {
        id: 111_368_241,
        source_type: SourceType::Wikidata,
        name: "Tiny Vector Graphics",
        extensions: &["tvg"],
        media_types: &["image/tinyvg"],
        signatures: &[],
        related_formats: &[],
    },
};
