use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207102: FileFormat = FileFormat {
    id: 28_207_102,
    puid: "wikidata/28207102",
    name: "The Print Shop Names file",
    extensions: &["nam"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
