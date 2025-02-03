use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27966933: FileFormat = FileFormat {
    id: 27_966_933,
    source_type: SourceType::Wikidata,
    name: "SID",
    extensions: &["psid", "sid"],
    media_types: &["audio/x-psid"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x49, 0x44])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x49, 0x44])],
                },
            }],
        },
    ],
    related_formats: &[],
};
