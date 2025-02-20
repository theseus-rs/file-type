use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_114238261: FileType = FileType {
    file_format: &FileFormat {
        id: 114_238_261,
        source_type: SourceType::Wikidata,
        name: "Streaming Audio Metafile",
        extensions: &["lam"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
