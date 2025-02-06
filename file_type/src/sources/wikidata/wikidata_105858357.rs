use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858357: FileFormat = FileFormat {
    id: 105_858_357,
    source_type: SourceType::Wikidata,
    name: "MovieShop Operators",
    extensions: &["ext"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xFF, 0x01, 0x00, 0x0C, 0x4D, 0x53, 0x4F, 0x50,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
