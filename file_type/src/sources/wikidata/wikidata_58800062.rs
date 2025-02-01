use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58800062: FileFormat = FileFormat {
    id: 58_800_062,
    puid: "wikidata/58800062",
    name: "PowerProject",
    extensions: &["pp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
