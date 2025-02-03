use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_821830: FileFormat = FileFormat {
    id: 821_830,
    source_type: SourceType::Wikidata,
    name: "Symbolic Link",
    extensions: &["slk", "sylk"],
    media_types: &["application/x-sylk"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x3B, 0x50])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x44, 0x3B, 0x50])],
                },
            }],
        },
    ],
    related_formats: &[],
};
