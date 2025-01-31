use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_4677626: FileFormat = FileFormat {
    id: 4_677_626,
    puid: "wikidata/4677626",
    name: "Activity Streams",
    extensions: &["json"],
    media_types: &["application/activity+json"],
    internal_signatures: &[],
    related_formats: &[],
};
