use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856995: FileFormat = FileFormat {
    id: 105_856_995,
    source_type: SourceType::Wikidata,
    name: "GUEmap document",
    extensions: &["gmp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xC7, 0x55, 0xC5, 0x6D, 0xE1, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
