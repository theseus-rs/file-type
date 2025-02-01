use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_129177072: FileFormat = FileFormat {
    id: 129_177_072,
    puid: "wikidata/129177072",
    name: "Fantom source code file",
    extensions: &["fan"],
    media_types: &["application/x-fantom"],
    internal_signatures: &[],
    related_formats: &[],
};
