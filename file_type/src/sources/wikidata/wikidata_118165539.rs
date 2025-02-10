use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_118165539: FileType = FileType {
    file_format: &FileFormat {
        id: 118_165_539,
        source_type: SourceType::Wikidata,
        name: "FotoFinish Image Format",
        extensions: &["sph"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
