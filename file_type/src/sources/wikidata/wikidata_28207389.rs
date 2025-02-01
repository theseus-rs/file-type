use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207389: FileFormat = FileFormat {
    id: 28_207_389,
    puid: "wikidata/28207389",
    name: "TIM",
    extensions: &["tim"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
