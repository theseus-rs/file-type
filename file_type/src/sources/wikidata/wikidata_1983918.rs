use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1983918: FileType = FileType {
    file_format: &FileFormat {
        id: 1_983_918,
        source_type: SourceType::Wikidata,
        name: "NEXUS",
        extensions: &["nex", "nxs"],
        media_types: &["text/plain"],
        signatures: &[],
        related_formats: &[],
    },
};
