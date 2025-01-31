use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854813: FileFormat = FileFormat {
    id: 105_854_813,
    puid: "wikidata/105854813",
    name: "DURILCA compressed file",
    extensions: &["dur"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA4, 0x93])],
            },
        }],
    }],
    related_formats: &[],
};
