use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27979408: FileFormat = FileFormat {
    id: 27_979_408,
    puid: "wikidata/27979408",
    name: "XNG",
    extensions: &["xng"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
