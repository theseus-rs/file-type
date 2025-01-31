use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_16342: FileFormat = FileFormat {
    id: 16_342,
    puid: "wikidata/16342",
    name: "XML Schema",
    extensions: &["xsd", "xsd"],
    media_types: &["application/xml", "text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
