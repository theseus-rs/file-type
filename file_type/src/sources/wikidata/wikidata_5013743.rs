use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_5013743: FileFormat = FileFormat {
    id: 5_013_743,
    source_type: SourceType::Wikidata,
    name: "Corel Photo-Paint Image",
    extensions: &["cpt"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x50, 0x54, 0x46, 0x49, 0x4C, 0x45])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x50, 0x54, 0x37, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
