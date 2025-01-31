use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83868375: FileFormat = FileFormat {
    id: 83_868_375,
    puid: "wikidata/83868375",
    name: "SOSI, version 4.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[],
    related_formats: &[],
};
