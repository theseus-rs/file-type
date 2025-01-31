use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128596042: FileFormat = FileFormat {
    id: 128_596_042,
    puid: "wikidata/128596042",
    name: "Aheui file format",
    extensions: &["aheui"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
