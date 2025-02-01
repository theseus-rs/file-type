use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_86920: FileFormat = FileFormat {
    id: 86_920,
    puid: "wikidata/86920",
    name: "text file",
    extensions: &["text", "txt"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
