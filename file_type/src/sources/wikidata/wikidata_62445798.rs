use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_62445798: FileType = FileType {
    file_format: &FileFormat {
        id: 62_445_798,
        source_type: SourceType::Wikidata,
        name: "OWL XML Serialization",
        extensions: &["owx"],
        media_types: &["application/owl+xml"],
        signatures: &[],
        related_formats: &[],
    },
};
