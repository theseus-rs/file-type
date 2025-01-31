use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_268086: FileFormat = FileFormat {
    id: 268_086,
    puid: "wikidata/268086",
    name: "OMDoc",
    extensions: &["omdoc"],
    media_types: &["application/omdoc+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
