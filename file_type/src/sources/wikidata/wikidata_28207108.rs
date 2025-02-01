use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207108: FileFormat = FileFormat {
    id: 28_207_108,
    puid: "wikidata/28207108",
    name: "The Print Shop Graphics file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
