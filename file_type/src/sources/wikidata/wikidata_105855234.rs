use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855234: FileFormat = FileFormat {
    id: 105_855_234,
    source_type: SourceType::Wikidata,
    name: "FIGfont",
    extensions: &["flf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x66, 0x6C, 0x66, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
