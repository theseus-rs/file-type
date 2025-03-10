use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_5379518: FileType = FileType {
    file_format: &FileFormat {
        id: 5_379_518,
        source_type: SourceType::Wikidata,
        name: "enriched text",
        extensions: &[],
        media_types: &["text/enriched"],
        signatures: &[],
        related_formats: &[],
    },
};
