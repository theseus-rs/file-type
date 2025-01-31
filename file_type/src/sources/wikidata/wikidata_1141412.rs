use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_1141412: FileFormat = FileFormat {
    id: 1_141_412,
    puid: "wikidata/1141412",
    name: "INI file",
    extensions: &["ini"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
