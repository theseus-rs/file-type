use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207561: FileFormat = FileFormat {
    id: 28_207_561,
    puid: "wikidata/28207561",
    name: "Xim",
    extensions: &["xim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
