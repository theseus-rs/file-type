use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858181: FileFormat = FileFormat {
    id: 105_858_181,
    source_type: SourceType::Wikidata,
    name: "Encapsulated PostScript binary",
    extensions: &["eps", "ept"],
    media_types: &["image/x-eps"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC5, 0xD0, 0xD3, 0xC6])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xC5, 0xD0, 0xD3, 0xC6])],
                },
            }],
        },
    ],
    related_formats: &[],
};
