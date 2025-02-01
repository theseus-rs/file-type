use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_112581715: FileFormat = FileFormat {
    id: 112_581_715,
    puid: "wikidata/112581715",
    name: "WAN",
    extensions: &["wan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
