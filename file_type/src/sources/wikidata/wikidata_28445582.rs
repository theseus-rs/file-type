use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445582: FileFormat = FileFormat {
    id: 28_445_582,
    puid: "wikidata/28445582",
    name: "AGSC",
    extensions: &["agsc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
