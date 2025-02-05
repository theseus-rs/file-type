use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858492: FileFormat = FileFormat {
    id: 105_858_492,
    source_type: SourceType::Wikidata,
    name: "Cybiko Cybook eBook",
    extensions: &["book"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x79, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
