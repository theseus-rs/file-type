use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207495: FileFormat = FileFormat {
    id: 28_207_495,
    puid: "wikidata/28207495",
    name: "Wigmore Artist 64",
    extensions: &["a64", "wig"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
