use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111317689: FileFormat = FileFormat {
    id: 111_317_689,
    puid: "wikidata/111317689",
    name: "Miles Sound System DLS 1 + XMI file",
    extensions: &["mss"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
