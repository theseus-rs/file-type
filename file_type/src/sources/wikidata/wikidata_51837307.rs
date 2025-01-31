use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51837307: FileFormat = FileFormat {
    id: 51_837_307,
    puid: "wikidata/51837307",
    name: "IBM DisplayWrite DCA Text File",
    extensions: &["dca"],
    media_types: &["application/dca-rft"],
    internal_signatures: &[],
    related_formats: &[],
};
