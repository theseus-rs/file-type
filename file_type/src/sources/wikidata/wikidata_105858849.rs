use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858849: FileFormat = FileFormat {
    id: 105_858_849,
    puid: "wikidata/105858849",
    name: "PMG Designer bitmap",
    extensions: &["pmd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF0, 0xED, 0xE4])],
            },
        }],
    }],
    related_formats: &[],
};
