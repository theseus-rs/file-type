use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205511: FileFormat = FileFormat {
    id: 28_205_511,
    puid: "wikidata/28205511",
    name: "HP 100LX/200LX icon",
    extensions: &["icn", "xbg"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
