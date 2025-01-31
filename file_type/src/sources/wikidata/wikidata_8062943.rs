use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_8062943: FileFormat = FileFormat {
    id: 8_062_943,
    puid: "wikidata/8062943",
    name: "ZAP File",
    extensions: &["zap"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
