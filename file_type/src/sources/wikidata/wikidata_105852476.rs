use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105852476: FileFormat = FileFormat {
    id: 105_852_476,
    source_type: SourceType::Wikidata,
    name: "Atari Works Database",
    extensions: &["std"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x31, 0x30, 0x00, 0x03])],
            },
        }],
    }],
    related_formats: &[],
};
