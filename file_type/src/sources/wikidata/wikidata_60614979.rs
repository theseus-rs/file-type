use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_60614979: FileFormat = FileFormat {
    id: 60_614_979,
    puid: "wikidata/60614979",
    name: "Serif DrawPlus Drawing, version 4",
    extensions: &["dpp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
