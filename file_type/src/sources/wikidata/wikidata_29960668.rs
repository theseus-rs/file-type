use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29960668: FileFormat = FileFormat {
    id: 29_960_668,
    puid: "wikidata/29960668",
    name: "RenderWare binary stream file",
    extensions: &["dff", "txd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
