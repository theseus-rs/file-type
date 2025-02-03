use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856977: FileFormat = FileFormat {
    id: 105_856_977,
    source_type: SourceType::Wikidata,
    name: "GNU Gettext Machine Object (litte endian)",
    extensions: &["gmo", "mo"],
    media_types: &["application/x-gettext-translation"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0x12, 0x04, 0x95])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xDE, 0x12, 0x04, 0x95])],
                },
            }],
        },
    ],
    related_formats: &[],
};
