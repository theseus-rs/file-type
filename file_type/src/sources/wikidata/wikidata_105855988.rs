use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855988: FileFormat = FileFormat {
    id: 105_855_988,
    source_type: SourceType::Wikidata,
    name: "Q and A Document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x42, 0x57, 0x50, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
