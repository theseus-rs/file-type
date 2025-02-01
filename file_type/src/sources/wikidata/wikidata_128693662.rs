use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_128693662: FileFormat = FileFormat {
    id: 128_693_662,
    puid: "wikidata/128693662",
    name: "Befunge file format",
    extensions: &["befunge"],
    media_types: &["application/x-befunge"],
    internal_signatures: &[],
    related_formats: &[],
};
