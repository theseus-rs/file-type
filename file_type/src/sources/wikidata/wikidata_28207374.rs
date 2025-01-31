use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207374: FileFormat = FileFormat {
    id: 28_207_374,
    puid: "wikidata/28207374",
    name: "Technicolor Dream COL",
    extensions: &["col"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
