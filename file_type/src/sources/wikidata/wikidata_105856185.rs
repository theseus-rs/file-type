use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856185: FileFormat = FileFormat {
    id: 105_856_185,
    puid: "wikidata/105856185",
    name: "MPlayer font Description",
    extensions: &["desc"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
