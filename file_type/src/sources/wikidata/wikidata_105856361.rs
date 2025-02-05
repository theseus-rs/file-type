use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856361: FileFormat = FileFormat {
    id: 105_856_361,
    source_type: SourceType::Wikidata,
    name: "CHEMICAL molecule Data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x63, 0x68, 0x65, 0x6D, 0x69, 0x63, 0x61, 0x6C, 0x5F, 0x6E, 0x61, 0x6D, 0x65,
                    0x28, 0x22,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
