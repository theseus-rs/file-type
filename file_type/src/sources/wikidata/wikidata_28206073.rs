use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206073: FileFormat = FileFormat {
    id: 28_206_073,
    puid: "wikidata/28206073",
    name: "Fuckpaint PI4",
    extensions: &["PI4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
