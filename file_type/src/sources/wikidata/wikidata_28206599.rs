use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206599: FileFormat = FileFormat {
    id: 28_206_599,
    puid: "wikidata/28206599",
    name: "MIX",
    extensions: &["mix"],
    media_types: &["image/vnd.mix"],
    internal_signatures: &[],
    related_formats: &[],
};
