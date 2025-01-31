use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854246: FileFormat = FileFormat {
    id: 105_854_246,
    puid: "wikidata/105854246",
    name: "AES Crypt encrypted",
    extensions: &["aes"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x45, 0x53, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
