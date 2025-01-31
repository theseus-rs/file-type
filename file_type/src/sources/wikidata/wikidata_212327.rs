use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_212327: FileFormat = FileFormat {
    id: 212_327,
    puid: "wikidata/212327",
    name: "Document Type Definition",
    extensions: &["dtd"],
    media_types: &["application/xml-dtd"],
    internal_signatures: &[],
    related_formats: &[],
};
