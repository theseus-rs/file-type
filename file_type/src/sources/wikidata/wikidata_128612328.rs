use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128612328: FileFormat = FileFormat {
    id: 128_612_328,
    puid: "wikidata/128612328",
    name: "Arturo file format",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
