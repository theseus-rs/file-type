use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858498: FileFormat = FileFormat {
    id: 105_858_498,
    source_type: SourceType::Wikidata,
    name: "LSS16 SYSLINUX Splash image",
    extensions: &["16", "lss"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0xF3, 0x13, 0x14])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x3D, 0xF3, 0x13, 0x14])],
                },
            }],
        },
    ],
    related_formats: &[],
};
