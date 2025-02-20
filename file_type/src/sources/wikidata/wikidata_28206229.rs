use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28206229: FileType = FileType {
    file_format: &FileFormat {
        id: 28_206_229,
        source_type: SourceType::Wikidata,
        name: "Gridded Binary",
        extensions: &["grb"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
