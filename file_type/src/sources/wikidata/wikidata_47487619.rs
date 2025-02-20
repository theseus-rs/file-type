use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47487619: FileType = FileType {
    file_format: &FileFormat {
        id: 47_487_619,
        source_type: SourceType::Wikidata,
        name: "GEM Metafile",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
