use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206916: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_916,
        source_type: SourceType::Wikidata,
        name: "Portfolio Graphics",
        extensions: &["pgf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
