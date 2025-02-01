use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207305: FileFormat = FileFormat {
    id: 28_207_305,
    puid: "wikidata/28207305",
    name: "True Colour Picture",
    extensions: &["trp", "tru"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
