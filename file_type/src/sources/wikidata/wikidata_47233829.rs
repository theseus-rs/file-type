use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_47233829: FileFormat = FileFormat {
    id: 47_233_829,
    puid: "wikidata/47233829",
    name: "L3B",
    extensions: &["l3b"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
