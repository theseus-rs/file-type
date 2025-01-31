use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28758212: FileFormat = FileFormat {
    id: 28_758_212,
    puid: "wikidata/28758212",
    name: "Street Atlas USA Draw File",
    extensions: &["an1"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
