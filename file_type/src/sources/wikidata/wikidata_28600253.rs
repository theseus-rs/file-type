use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600253: FileFormat = FileFormat {
    id: 28_600_253,
    puid: "wikidata/28600253",
    name: ".art",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
