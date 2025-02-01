use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_122229772: FileFormat = FileFormat {
    id: 122_229_772,
    puid: "wikidata/122229772",
    name: "Digital Interface Format",
    extensions: &["dif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
