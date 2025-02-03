use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859142: FileFormat = FileFormat {
    id: 105_859_142,
    source_type: SourceType::Wikidata,
    name: "Madagascar: Escape 2 Africa game data archive",
    extensions: &["arc", "bld"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x47, 0x41, 0x1A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x49, 0x47, 0x41, 0x1A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
