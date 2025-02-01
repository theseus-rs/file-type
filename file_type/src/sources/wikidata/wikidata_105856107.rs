use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856107: FileFormat = FileFormat {
    id: 105_856_107,
    puid: "wikidata/105856107",
    name: "Dis bytecode (signed)",
    extensions: &["dis"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x0E, 0x17, 0x22])],
            },
        }],
    }],
    related_formats: &[],
};
