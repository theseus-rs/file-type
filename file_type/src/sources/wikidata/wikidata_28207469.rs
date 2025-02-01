use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207469: FileFormat = FileFormat {
    id: 28_207_469,
    puid: "wikidata/28207469",
    name: "Vivid IMG",
    extensions: &["img"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
