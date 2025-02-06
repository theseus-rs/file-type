use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_979630: FileFormat = FileFormat {
    id: 979_630,
    source_type: SourceType::Wikidata,
    name: "Industry Foundation Classes",
    extensions: &["ifc", "ifcXML", "ifczip"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
