use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_131330520: FileType = FileType {
    file_format: &FileFormat {
        id: 131_330_520,
        source_type: SourceType::Wikidata,
        name: "Typographic Number Theory file format",
        extensions: &["tnt"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
