use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130634071: FileFormat = FileFormat {
    id: 130_634_071,
    puid: "wikidata/130634071",
    name: "RITA file format",
    extensions: &["rita"],
    media_types: &["text/rita"],
    internal_signatures: &[],
    related_formats: &[],
};
