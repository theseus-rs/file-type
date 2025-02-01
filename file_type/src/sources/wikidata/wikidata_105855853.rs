use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855853: FileFormat = FileFormat {
    id: 105_855_853,
    puid: "wikidata/105855853",
    name: "SMS ASCII Dataset",
    extensions: &["dat"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x41, 0x54, 0x41, 0x53, 0x45, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
