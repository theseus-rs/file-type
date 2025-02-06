use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_212327: FileFormat = FileFormat {
    id: 212_327,
    source_type: SourceType::Wikidata,
    name: "Document Type Definition",
    extensions: &["dtd"],
    media_types: &["application/xml-dtd"],
    signatures: &[],
    related_formats: &[],
};
