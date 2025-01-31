use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858113: FileFormat = FileFormat {
    id: 105_858_113,
    puid: "wikidata/105858113",
    name: "Infinity Engine configuration",
    extensions: &["ini"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5B, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x73, 0x5D, 0x0D, 0x0A, 0x0D, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
