use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109944440: FileFormat = FileFormat {
    id: 109_944_440,
    puid: "wikidata/109944440",
    name: "CadKey file format",
    extensions: &["prt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
