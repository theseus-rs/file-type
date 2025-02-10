use crate::format::{FileFormat, SourceType};
use crate::FileType;

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
