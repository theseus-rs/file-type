use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_979630: FileType = FileType {
    file_format: &FileFormat {
        id: 979_630,
        source_type: SourceType::Wikidata,
        name: "Industry Foundation Classes",
        extensions: &["ifc", "ifcXML", "ifczip"],
        media_types: &["application/octet-stream"],
        signatures: &[],
        related_formats: &[],
    },
};
