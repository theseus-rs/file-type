use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207114: FileFormat = FileFormat {
    id: 28_207_114,
    puid: "wikidata/28207114",
    name: "The New Print Shop Names file",
    extensions: &["pnm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
