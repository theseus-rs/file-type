use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858406: FileFormat = FileFormat {
    id: 105_858_406,
    puid: "wikidata/105858406",
    name: "EOP Music Master music score",
    extensions: &["eopm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x76, 0x65, 0x72, 0x79, 0x6F, 0x6E, 0x65, 0x50, 0x69, 0x61, 0x6E, 0x6F,
                    0x00, 0x00, 0x00, 0xC9, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
