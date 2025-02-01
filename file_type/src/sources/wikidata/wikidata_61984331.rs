use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_61984331: FileFormat = FileFormat {
    id: 61_984_331,
    puid: "wikidata/61984331",
    name: "FoxPro Project",
    extensions: &["pjx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
