use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27826392: FileFormat = FileFormat {
    id: 27_826_392,
    puid: "wikidata/27826392",
    name: "Proxy Unrestricted Access Image",
    extensions: &["uai"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
