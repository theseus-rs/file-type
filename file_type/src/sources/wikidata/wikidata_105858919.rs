use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858919: FileFormat = FileFormat {
    id: 105_858_919,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff format bitmap (low-res anim)",
    extensions: &["tn4", "tny"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x07, 0x77])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x03, 0x07, 0x77])],
                },
            }],
        },
    ],
    related_formats: &[],
};
