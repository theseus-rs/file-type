use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117448727: FileFormat = FileFormat {
    id: 117_448_727,
    puid: "wikidata/117448727",
    name: "Transcriber AG TAG Format",
    extensions: &["tag"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
