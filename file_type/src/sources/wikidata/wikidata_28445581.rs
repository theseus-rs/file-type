use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445581: FileFormat = FileFormat {
    id: 28_445_581,
    puid: "wikidata/28445581",
    name: "ADRIFT",
    extensions: &["taf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
