use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_110016661: FileType = FileType {
    file_format: &FileFormat {
        id: 110_016_661,
        source_type: SourceType::Wikidata,
        name: "TEI P4 XML - Corpus File",
        extensions: &["odd", "tei", "xml"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
