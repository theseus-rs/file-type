use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967347: FileFormat = FileFormat {
    id: 27_967_347,
    source_type: SourceType::Wikidata,
    name: "Sibelius Score",
    extensions: &["si7", "sib"],
    media_types: &[],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53,
                    ])],
                },
            }],
        },
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x0F, 0x53, 0x49, 0x42, 0x45, 0x4C, 0x49, 0x55, 0x53,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
