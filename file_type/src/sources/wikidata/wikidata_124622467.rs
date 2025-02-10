use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_124622467: FileType = FileType {
    file_format: &FileFormat {
        id: 124_622_467,
        source_type: SourceType::Wikidata,
        name: "TEI/XML",
        extensions: &["odd", "xml"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
