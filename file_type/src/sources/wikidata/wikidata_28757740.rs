use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28757740: FileType = FileType {
    file_format: &FileFormat {
        id: 28_757_740,
        source_type: SourceType::Wikidata,
        name: "GEM VDI Metafile",
        extensions: &["gem"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
