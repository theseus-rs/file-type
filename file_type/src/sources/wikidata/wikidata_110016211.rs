use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110016211: FileType = FileType {
    file_format: &FileFormat {
        id: 110_016_211,
        source_type: SourceType::Wikidata,
        name: "TEI P4 XML - Single Text File",
        extensions: &["odd", "tei", "xml"],
        media_types: &["application/tei+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
