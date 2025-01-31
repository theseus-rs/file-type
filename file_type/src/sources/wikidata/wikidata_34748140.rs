use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_34748140: FileFormat = FileFormat {
    id: 34_748_140,
    puid: "wikidata/34748140",
    name: "TAP",
    extensions: &["tap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
