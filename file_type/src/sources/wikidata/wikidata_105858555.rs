use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858555: FileFormat = FileFormat {
    id: 105_858_555,
    source_type: SourceType::Wikidata,
    name: "Dore Raster bitmap",
    extensions: &["dore", "img"],
    media_types: &["text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x61, 0x73, 0x74, 0x65, 0x72, 0x74, 0x79, 0x70, 0x65,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x72, 0x61, 0x73, 0x74, 0x65, 0x72, 0x74, 0x79, 0x70, 0x65,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
