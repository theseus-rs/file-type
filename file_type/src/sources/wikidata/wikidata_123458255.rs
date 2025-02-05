use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_123458255: FileFormat = FileFormat {
    id: 123_458_255,
    source_type: SourceType::Wikidata,
    name: "Apple Archive",
    extensions: &["aar"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x70, 0x62, 0x7A, 0x65])],
            },
        }],
    }],
    related_formats: &[],
};
