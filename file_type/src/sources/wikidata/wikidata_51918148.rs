use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51918148: FileFormat = FileFormat {
    id: 51_918_148,
    puid: "wikidata/51918148",
    name: "XYWrite Document",
    extensions: &["xy"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
