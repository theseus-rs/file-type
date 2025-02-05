use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858061: FileFormat = FileFormat {
    id: 105_858_061,
    source_type: SourceType::Wikidata,
    name: "Epic ITS format",
    extensions: &["int"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x45, 0x70, 0x69, 0x63, 0x20, 0x49, 0x54, 0x53, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
