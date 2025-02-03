use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855861: FileFormat = FileFormat {
    id: 105_855_861,
    source_type: SourceType::Wikidata,
    name: "Delphi Project source",
    extensions: &["dpr"],
    media_types: &["text/plain", "text/x-pascal"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20,
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
                        0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D, 0x20,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
