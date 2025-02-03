use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_285972: FileFormat = FileFormat {
    id: 285_972,
    source_type: SourceType::Wikidata,
    name: "Mbox",
    extensions: &["mbox", "mbx"],
    media_types: &["application/mbox"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x72, 0x6F, 0x6D, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
