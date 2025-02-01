use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856440: FileFormat = FileFormat {
    id: 105_856_440,
    puid: "wikidata/105856440",
    name: "Winbot Script",
    extensions: &["wbs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
