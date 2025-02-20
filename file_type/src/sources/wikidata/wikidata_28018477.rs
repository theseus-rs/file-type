use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28018477: FileType = FileType {
    file_format: &FileFormat {
        id: 28_018_477,
        source_type: SourceType::Wikidata,
        name: "Indeo Video Format",
        extensions: &["ivf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
