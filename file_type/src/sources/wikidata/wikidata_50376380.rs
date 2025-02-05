use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_50376380: FileFormat = FileFormat {
    id: 50_376_380,
    source_type: SourceType::Wikidata,
    name: "TZX Format",
    extensions: &["cdt", "tzx"],
    media_types: &["application/x-spectrum-tzx"],
    signatures: &[
        Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x5A, 0x58, 0x54, 0x61, 0x70, 0x65, 0x21, 0x1A,
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
                        0x5A, 0x58, 0x54, 0x61, 0x70, 0x65, 0x21, 0x1A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
