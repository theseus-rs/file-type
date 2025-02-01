use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7903498: FileFormat = FileFormat {
    id: 7_903_498,
    puid: "wikidata/7903498",
    name: "UTZ",
    extensions: &["utz"],
    media_types: &["application/vnd.uiq.theme"],
    internal_signatures: &[],
    related_formats: &[],
};
