use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206066: FileFormat = FileFormat {
    id: 28_206_066,
    puid: "wikidata/28206066",
    name: "View ST/TT TT-Low",
    extensions: &["PI4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
