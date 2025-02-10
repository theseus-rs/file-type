use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_129414825: FileType = FileType {
    file_format: &FileFormat {
        id: 129_414_825,
        source_type: SourceType::Wikidata,
        name: "Golo source code file",
        extensions: &["golo"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
