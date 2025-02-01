use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128583427: FileFormat = FileFormat {
    id: 128_583_427,
    puid: "wikidata/128583427",
    name: "ABAP file format",
    extensions: &["abap"],
    media_types: &["text/x-abap"],
    internal_signatures: &[],
    related_formats: &[],
};
