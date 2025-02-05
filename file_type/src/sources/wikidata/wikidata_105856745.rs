use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856745: FileFormat = FileFormat {
    id: 105_856_745,
    source_type: SourceType::Wikidata,
    name: "Wii U8 archive",
    extensions: &["u8"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0xAA, 0x38, 0x2D])],
            },
        }],
    }],
    related_formats: &[],
};
