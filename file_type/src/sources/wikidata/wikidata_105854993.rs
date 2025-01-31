use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854993: FileFormat = FileFormat {
    id: 105_854_993,
    puid: "wikidata/105854993",
    name: "Micrognosis compressed archive",
    extensions: &["mar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2D, 0x61, 0x68, 0x30, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
