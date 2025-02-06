use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105851903: FileFormat = FileFormat {
    id: 105_851_903,
    source_type: SourceType::Wikidata,
    name: "DataShow Sprite",
    extensions: &["spr"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x0D, 0x44, 0x61, 0x74, 0x61, 0x53, 0x68, 0x6F, 0x77, 0x20, 0x49, 0x63, 0x6F,
                    0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
