use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855052: FileFormat = FileFormat {
    id: 105_855_052,
    source_type: SourceType::Wikidata,
    name: "Avro Object Container File",
    extensions: &["avro"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4F, 0x62, 0x6A, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
