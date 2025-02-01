use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105865336: FileFormat = FileFormat {
    id: 105_865_336,
    puid: "wikidata/105865336",
    name: "PC-Axis data (var 1)",
    extensions: &["px"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
