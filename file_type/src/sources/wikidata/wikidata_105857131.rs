use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857131: FileFormat = FileFormat {
    id: 105_857_131,
    source_type: SourceType::Wikidata,
    name: "GNU Gettext Machine Object (big endian)",
    extensions: &["gmo", "mo"],
    media_types: &["application/x-gettext-translation"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x95, 0x04, 0x12, 0xDE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x95, 0x04, 0x12, 0xDE])],
                },
            }],
        },
    ],
    related_formats: &[],
};
