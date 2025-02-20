use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_47487623: FileType = FileType {
    file_format: &FileFormat {
        id: 47_487_623,
        source_type: SourceType::Wikidata,
        name: "GEM Metafile",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
