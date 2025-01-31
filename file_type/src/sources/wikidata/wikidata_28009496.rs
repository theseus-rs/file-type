use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009496: FileFormat = FileFormat {
    id: 28_009_496,
    puid: "wikidata/28009496",
    name: "Zelda Solarus DX saved game",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
