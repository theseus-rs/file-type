use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_10465505: FileFormat = FileFormat {
    id: 10_465_505,
    puid: "wikidata/10465505",
    name: "DTS-HD",
    extensions: &["dtshd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
