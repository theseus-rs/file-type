use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27967451: FileFormat = FileFormat {
    id: 27_967_451,
    puid: "wikidata/27967451",
    name: "GRASP GL",
    extensions: &["gl"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
