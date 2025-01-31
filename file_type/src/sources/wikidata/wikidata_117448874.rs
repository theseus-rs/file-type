use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117448874: FileFormat = FileFormat {
    id: 117_448_874,
    puid: "wikidata/117448874",
    name: "Transcriber TRS Format",
    extensions: &["trs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
