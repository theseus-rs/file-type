use crate::FileType;
use crate::format::{FileFormat, SourceType};

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
