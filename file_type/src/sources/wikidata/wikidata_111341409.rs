use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341409: FileFormat = FileFormat {
    id: 111_341_409,
    puid: "wikidata/111341409",
    name: "Yamaha EX5 'all' format",
    extensions: &["s1a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
