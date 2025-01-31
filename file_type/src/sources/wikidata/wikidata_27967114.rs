use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967114: FileFormat = FileFormat {
    id: 27_967_114,
    puid: "wikidata/27967114",
    name: "Arkos Tracker",
    extensions: &["aks"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
