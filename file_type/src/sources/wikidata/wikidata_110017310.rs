use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110017310: FileType = FileType {
    file_format: &FileFormat {
        id: 110_017_310,
        source_type: SourceType::Wikidata,
        name: "TEI P5 XML - Corpus File",
        extensions: &["odd", "tei", "xml"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
