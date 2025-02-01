use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51839192: FileFormat = FileFormat {
    id: 51_839_192,
    puid: "wikidata/51839192",
    name: "PHP Script Page",
    extensions: &["php"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
