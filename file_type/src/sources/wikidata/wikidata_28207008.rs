use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207008: FileFormat = FileFormat {
    id: 28_207_008,
    puid: "wikidata/28207008",
    name: "Picture Publisher 4",
    extensions: &["pp4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
