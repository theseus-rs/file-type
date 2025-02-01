use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867323: FileFormat = FileFormat {
    id: 105_867_323,
    puid: "wikidata/105867323",
    name: "OS/2 Network Information File (with rem)",
    extensions: &["nif"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
