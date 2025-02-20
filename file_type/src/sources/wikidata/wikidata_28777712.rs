use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28777712: FileType = FileType {
    file_format: &FileFormat {
        id: 28_777_712,
        source_type: SourceType::Wikidata,
        name: "NFF",
        extensions: &["nff"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
