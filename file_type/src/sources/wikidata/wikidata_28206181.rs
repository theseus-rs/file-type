use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206181: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_181,
        source_type: SourceType::Wikidata,
        name: "GIMP Parametric Brush",
        extensions: &["vbr"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
