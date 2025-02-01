use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207503: FileFormat = FileFormat {
    id: 28_207_503,
    puid: "wikidata/28207503",
    name: "WinMiPS",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
