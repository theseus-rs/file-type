use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4545411: FileFormat = FileFormat {
    id: 4_545_411,
    puid: "wikidata/4545411",
    name: "Blizzard Game Picture",
    extensions: &["blp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
