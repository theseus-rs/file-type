use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009488: FileFormat = FileFormat {
    id: 28_009_488,
    puid: "wikidata/28009488",
    name: "Tibia map file",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
