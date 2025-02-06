use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105864637: FileFormat = FileFormat {
    id: 105_864_637,
    source_type: SourceType::Wikidata,
    name: "PhysicsEditor Sheet",
    extensions: &["pes"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x59, 0x45, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
