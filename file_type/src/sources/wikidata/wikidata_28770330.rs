use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28770330: FileFormat = FileFormat {
    id: 28_770_330,
    source_type: SourceType::Wikidata,
    name: "LightWave Scene",
    extensions: &["lws", "scn"],
    media_types: &["image/x-lws"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x53, 0x43])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4C, 0x57, 0x53, 0x43])],
                },
            }],
        },
    ],
    related_formats: &[],
};
