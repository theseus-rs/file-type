use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110016938: FileType = FileType {
    file_format: &FileFormat {
        id: 110_016_938,
        source_type: SourceType::Wikidata,
        name: "TEI P5 - Single Text File",
        extensions: &["odd", "tei", "xml"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
