use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_5205563: FileType = FileType {
    file_format: &FileFormat {
        id: 5_205_563,
        source_type: SourceType::Wikidata,
        name: "Downloadable Sounds",
        extensions: &["dls"],
        media_types: &["audio/dls"],
        signatures: &[],
        related_formats: &[],
    },
};
