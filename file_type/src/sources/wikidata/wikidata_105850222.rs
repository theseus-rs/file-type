use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850222: FileFormat = FileFormat {
    id: 105_850_222,
    puid: "wikidata/105850222",
    name: "3ds UI colors",
    extensions: &["clr"],
    media_types: &["text/plain"],
    internal_signatures: &[],
    related_formats: &[],
};
