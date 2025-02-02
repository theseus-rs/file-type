use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858863: FileFormat = FileFormat {
    id: 105_858_863,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff format bitmap (med-res)",
    extensions: &["tn2", "tny"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x07, 0x77])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x01, 0x07, 0x77])],
                },
            }],
        },
    ],
    related_formats: &[],
};
