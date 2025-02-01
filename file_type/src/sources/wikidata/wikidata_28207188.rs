use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207188: FileFormat = FileFormat {
    id: 28_207_188,
    puid: "wikidata/28207188",
    name: "QDV",
    extensions: &["qdv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
