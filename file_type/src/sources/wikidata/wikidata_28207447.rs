use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207447: FileFormat = FileFormat {
    id: 28_207_447,
    puid: "wikidata/28207447",
    name: "VIPS",
    extensions: &["v"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
