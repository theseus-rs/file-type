use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_28346237: FileType = FileType {
    file_format: &FileFormat {
        id: 28_346_237,
        source_type: SourceType::Wikidata,
        name: "TTDDD",
        extensions: &["ttd"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
