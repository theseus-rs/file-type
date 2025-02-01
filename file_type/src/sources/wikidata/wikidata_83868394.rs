use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_83868394: FileFormat = FileFormat {
    id: 83_868_394,
    puid: "wikidata/83868394",
    name: "SOSI, version 8.1",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[],
    related_formats: &[],
};
