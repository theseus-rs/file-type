use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207125: FileFormat = FileFormat {
    id: 28_207_125,
    puid: "wikidata/28207125",
    name: "The New Print Shop Graphics file",
    extensions: &["pog"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
