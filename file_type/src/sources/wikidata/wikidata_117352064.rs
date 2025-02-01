use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117352064: FileFormat = FileFormat {
    id: 117_352_064,
    puid: "wikidata/117352064",
    name: "Capture Design",
    extensions: &["dsn"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
