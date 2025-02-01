use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205711: FileFormat = FileFormat {
    id: 28_205_711,
    puid: "wikidata/28205711",
    name: "STW",
    extensions: &["stw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
