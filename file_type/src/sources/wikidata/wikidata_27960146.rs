use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27960146: FileFormat = FileFormat {
    id: 27_960_146,
    puid: "wikidata/27960146",
    name: "X2A",
    extensions: &["x2a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
