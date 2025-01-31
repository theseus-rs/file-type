use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207427: FileFormat = FileFormat {
    id: 28_207_427,
    puid: "wikidata/28207427",
    name: "Verity Image Format",
    extensions: &["vif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
