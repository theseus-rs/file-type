use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849601: FileFormat = FileFormat {
    id: 105_849_601,
    puid: "wikidata/105849601",
    name: "Help File Contents",
    extensions: &["cnt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
