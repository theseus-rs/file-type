use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_28600763: FileType = FileType {
    file_format: &FileFormat {
        id: 28_600_763,
        source_type: SourceType::Wikidata,
        name: "Electronic Arts INF",
        extensions: &["inf"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
