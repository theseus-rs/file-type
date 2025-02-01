use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28009498: FileFormat = FileFormat {
    id: 28_009_498,
    puid: "wikidata/28009498",
    name: "Zj-Stream",
    extensions: &["prn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
