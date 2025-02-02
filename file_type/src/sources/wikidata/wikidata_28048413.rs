use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28048413: FileFormat = FileFormat {
    id: 28_048_413,
    source_type: SourceType::Wikidata,
    name: "Cineon",
    extensions: &["cin"],
    media_types: &["image/cineon", "image/x-cin"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x2A, 0x5F, 0xD7])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x80, 0x2A, 0x5F, 0xD7])],
                },
            }],
        },
    ],
    related_formats: &[],
};
