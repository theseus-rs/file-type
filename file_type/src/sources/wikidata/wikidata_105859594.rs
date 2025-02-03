use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859594: FileFormat = FileFormat {
    id: 105_859_594,
    source_type: SourceType::Wikidata,
    name: "Shrew VPN configuration",
    extensions: &["vpn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x3A])],
            },
        }],
    }],
    related_formats: &[],
};
